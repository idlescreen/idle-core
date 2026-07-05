// SPDX-License-Identifier: MIT

use std::path::Path;
use std::time::Duration;
use libloading::Library;
use trance_api::ScreensaverInstance;
use trance_upscaler::{FilterMode, FrameUpscaler, resolve_render_scale};

use crate::cell_renderer::CellRenderer;
use crate::launcher::{LaunchMode, resolve_saver_binary};

use super::{PluginGuard, PluginSession};

impl PluginSession {
    pub fn load(saver_name: &str) -> Result<Self, String> {
        Self::load_with_options(saver_name, &LaunchMode::Daemon, None, None)
    }

    pub fn load_with_options(
        saver_name: &str,
        launch_mode: &LaunchMode,
        gpu_enabled: Option<bool>,
        render_scale: Option<f32>,
    ) -> Result<Self, String> {
        let path =
            resolve_saver_binary(saver_name, launch_mode).map_err(|error| error.to_string())?;
        println!(
            "trance-runner: loading plugin '{}' from {}",
            saver_name,
            path.display()
        );
        Self::load_path_with_options(&path, gpu_enabled, render_scale)
    }

    pub fn load_path_with_options(
        path: &Path,
        gpu_enabled: Option<bool>,
        render_scale: Option<f32>,
    ) -> Result<Self, String> {
        let renderer = CellRenderer::new()?;
        let use_gpu = gpu_enabled.unwrap_or_else(trance_upscaler::gpu_enabled);
        let render_scale = resolve_render_scale(use_gpu, render_scale);
        let upscaler = FrameUpscaler::new(use_gpu, FilterMode::from_env());
        if upscaler.using_gpu() {
            unsafe {
                std::env::set_var("TRANCE_GPU_ACTIVE", "1");
            }
            println!(
                "trance-runner: GPU upscale enabled (render scale {:.0}%, adapter: {})",
                render_scale * 100.0,
                upscaler.adapter_name().unwrap_or("unknown")
            );
        } else {
            unsafe {
                std::env::remove_var("TRANCE_GPU_ACTIVE");
            }
            println!(
                "trance-runner: CPU upscale (render scale {:.0}%)",
                render_scale * 100.0
            );
        }

        unsafe {
            let lib = Library::new(path).map_err(|error| error.to_string())?;
            let create_fn: libloading::Symbol<unsafe extern "C" fn() -> *mut ScreensaverInstance> =
                lib.get(b"create_screensaver")
                    .map_err(|error| error.to_string())?;
            let destroy_fn: libloading::Symbol<unsafe extern "C" fn(*mut ScreensaverInstance)> =
                lib.get(b"destroy_screensaver")
                    .map_err(|error| error.to_string())?;

            let raw_ptr = create_fn();
            if raw_ptr.is_null() {
                return Err("plugin returned null screensaver instance".into());
            }

            let guard = PluginGuard {
                ptr: raw_ptr,
                destroy: *destroy_fn,
                _lib: lib,
            };

            Ok(Self {
                plugin: guard,
                renderer,
                upscaler,
                render_scale,
                grid: Vec::new(),
                content_buf: Vec::new(),
                pixel_buf: Vec::new(),
                physics_accumulator: Duration::ZERO,
                physics_duration: Duration::from_secs_f32(1.0 / 120.0),
                time_elapsed: Duration::ZERO,
                simulation_cols: 0,
                simulation_rows: 0,
                hardware_scaling: false,
            })
        }
    }
}
