// SPDX-License-Identifier: MIT
//! Phase helpers for [`super::render::render`]. Each helper handles
//! one step of the per-frame GPU pipeline; the orchestrator composes
//! them in order.

use super::gpu_init::{GpuCellRenderer, Uniforms};

impl GpuCellRenderer {
    /// Compute the row-aligned staging buffer size for the given
    /// content width in bytes.
    pub(super) fn compute_padded_size(&self, content_w: u32) -> (u32, u32) {
        let unpadded = content_w * 4;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padded = unpadded + (align - unpadded % align) % align;
        (unpadded, padded)
    }

    /// Recreate the render target texture and matching staging buffer
    /// when the content size has changed. Sets `recreate_bg = true`
    /// so the bind group gets rebuilt with the new texture view.
    pub(super) fn ensure_target_texture(
        &mut self,
        content_w: u32,
        content_h: u32,
        padded: u32,
        recreate_bg: &mut bool,
    ) {
        if self.target_width == content_w
            && self.target_height == content_h
            && self.texture.is_some()
        {
            return;
        }
        self.target_width = content_w;
        self.target_height = content_h;
        Self::ensure_texture(
            &self.device,
            &mut self.texture,
            "cell render target",
            content_w,
            content_h,
            wgpu::TextureFormat::Bgra8Unorm,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        );
        self.staging_buffer = Some(self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("staging"),
            size: (padded * content_h) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        }));
        *recreate_bg = true;
    }

    /// Recreate the atlas texture if the size has changed or the
    /// glyph set is dirty. Returns `(atlas_w, atlas_h, recreated)`.
    pub(super) fn ensure_atlas_texture(
        &mut self,
        atlas_cols: usize,
        atlas_rows: usize,
        cell_width: usize,
        cell_height: usize,
        dirty: bool,
    ) -> (usize, usize, bool) {
        let atlas_w = atlas_cols * cell_width;
        let atlas_h = atlas_rows * cell_height;
        if !dirty
            && self.atlas_texture.is_some()
            && self.atlas_width == atlas_w
            && self.atlas_height == atlas_h
        {
            return (atlas_w, atlas_h, false);
        }
        self.atlas_width = atlas_w;
        self.atlas_height = atlas_h;
        Self::ensure_texture(
            &self.device,
            &mut self.atlas_texture,
            "atlas",
            atlas_w as u32,
            atlas_h as u32,
            wgpu::TextureFormat::R8Unorm,
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        );
        (atlas_w, atlas_h, true)
    }

    /// Re-upload the atlas glyph image to the GPU.
    pub(super) fn upload_atlas(
        &self,
        atlas_tex: &wgpu::Texture,
        atlas_image: &[u8],
        atlas_w: usize,
        atlas_h: usize,
    ) {
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: atlas_tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            atlas_image,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(atlas_w as u32),
                rows_per_image: None,
            },
            wgpu::Extent3d {
                width: atlas_w as u32,
                height: atlas_h as u32,
                depth_or_array_layers: 1,
            },
        );
    }

    /// Rebuild the bind group with the current buffer / texture
    /// handles.
    pub(super) fn rebuild_bind_group(
        &mut self,
        uni_binding: wgpu::BindingResource<'_>,
        cells_binding: wgpu::BindingResource<'_>,
    ) {
        let Some(atlas) = self.atlas_texture.as_ref() else {
            return;
        };
        let atlas_view = atlas.create_view(&wgpu::TextureViewDescriptor::default());
        self.bind_group = Some(self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bind group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uni_binding,
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: cells_binding,
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&atlas_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&self.atlas_sampler),
                },
            ],
        }));
    }

    /// Write the per-frame uniforms buffer.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn upload_uniforms(
        &self,
        uni_buf: &wgpu::Buffer,
        cols: usize,
        rows: usize,
        cell_width: usize,
        cell_height: usize,
        atlas_cols: usize,
        atlas_rows: usize,
        scanlines: bool,
    ) {
        let uniforms = Uniforms {
            cols: cols as u32,
            rows: rows as u32,
            cell_width: cell_width as u32,
            cell_height: cell_height as u32,
            atlas_cols: atlas_cols as u32,
            atlas_rows: atlas_rows as u32,
            scanlines: u32::from(scanlines),
            padding: 0,
        };
        self.queue
            .write_buffer(uni_buf, 0, bytemuck::bytes_of(&uniforms));
    }

    /// Run the cell-rendering GPU pass.
    pub(super) fn run_render_pass(
        &self,
        target_tex: &wgpu::Texture,
        bind_gp: &wgpu::BindGroup,
        encoder: &mut wgpu::CommandEncoder,
        cols: usize,
        rows: usize,
    ) {
        let target_view = target_tex.create_view(&wgpu::TextureViewDescriptor::default());
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, bind_gp, &[]);
        render_pass.draw(0..6, 0..(cols * rows) as u32);
    }

    /// Encode a copy from the render target to the staging buffer.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn copy_texture_to_staging(
        &self,
        target_tex: &wgpu::Texture,
        staging_buf: &wgpu::Buffer,
        padded: u32,
        content_w: u32,
        content_h: u32,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: target_tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: staging_buf,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded),
                    rows_per_image: None,
                },
            },
            wgpu::Extent3d {
                width: content_w,
                height: content_h,
                depth_or_array_layers: 1,
            },
        );
    }
}
