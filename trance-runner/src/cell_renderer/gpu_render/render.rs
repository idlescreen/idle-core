// SPDX-License-Identifier: MIT

use super::gpu_init::{GpuCell, GpuCellRenderer, Uniforms};
use trance_api::TerminalCell;

impl GpuCellRenderer {
    /// Render one frame of the cell renderer.
    ///
    /// The method is a thin orchestrator over the phase helpers below
    /// (target-texture resize, atlas update, bind-group rebuild, GPU
    /// pass, staging copy). Splitting each phase out keeps the
    /// `render.rs` file under the standing 250-line cap.
    pub fn render(
        &mut self,
        grid: &[TerminalCell],
        grid_cols: usize,
        col_start: usize,
        row_start: usize,
        cols: usize,
        rows: usize,
        scanlines: bool,
        cell_width: usize,
        cell_height: usize,
        atlas_cols: usize,
        atlas_rows: usize,
        atlas_image: &[u8],
        atlas_dirty: bool,
        atlas_chars: &[char],
        out: &mut Vec<u8>,
    ) {
        let (content_w, content_h) = ((cols * cell_width) as u32, (rows * cell_height) as u32);
        if content_w == 0 || content_h == 0 {
            return;
        }

        let (unpadded, padded) = self.compute_padded_size(content_w);

        let mut recreate_bg = false;
        self.ensure_target_texture(content_w, content_h, padded, &mut recreate_bg);

        let cells_size = (cols * rows * std::mem::size_of::<GpuCell>()) as u64;
        let (cells_buf, c_re) = Self::ensure_buffer(
            &self.device,
            &mut self.cells_buffer,
            "cells",
            cells_size,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        );

        let (uni_buf, u_re) = Self::ensure_buffer(
            &self.device,
            &mut self.uniform_buffer,
            "uniforms",
            std::mem::size_of::<Uniforms>() as u64,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        );

        let (atlas_w, atlas_h, a_re) = self.ensure_atlas_texture(
            atlas_cols,
            atlas_rows,
            cell_width,
            cell_height,
            atlas_dirty,
        );
        let recreate_bind = recreate_bg || c_re || u_re || a_re;

        if (atlas_dirty || recreate_bind)
            && let Some(ref atlas_tex) = self.atlas_texture
        {
            self.upload_atlas(atlas_tex, atlas_image, atlas_w, atlas_h);
        }

        if recreate_bind || self.bind_group.is_none() {
            self.rebuild_bind_group(uni_buf.as_entire_binding(), cells_buf.as_entire_binding());
        }

        self.upload_uniforms(
            &uni_buf,
            cols,
            rows,
            cell_width,
            cell_height,
            atlas_cols,
            atlas_rows,
            scanlines,
        );

        let gpu_cells = super::gpu_cells::build_gpu_cells(
            grid,
            grid_cols,
            col_start,
            row_start,
            cols,
            rows,
            atlas_chars,
        );
        self.queue
            .write_buffer(&cells_buf, 0, bytemuck::cast_slice(&gpu_cells));

        let (Some(target_tex), Some(bind_gp), Some(staging_buf)) = (
            self.texture.as_ref(),
            self.bind_group.as_ref(),
            self.staging_buffer.as_ref(),
        ) else {
            return;
        };

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render"),
            });
        self.run_render_pass(target_tex, bind_gp, &mut encoder, cols, rows);

        self.copy_texture_to_staging(
            target_tex,
            staging_buf,
            padded,
            content_w,
            content_h,
            &mut encoder,
        );

        self.queue.submit(std::iter::once(encoder.finish()));

        if let Some(ref buf) = self.staging_buffer {
            super::gpu_cells::copy_staging_to_out(
                buf,
                &self.device,
                content_w,
                content_h,
                unpadded,
                padded,
                out,
            );
        }
    }
}
