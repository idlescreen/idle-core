# Codebase File Line Limits

This project enforces a range of **25 to 250 lines** per source file to ensure readability and compatibility with smaller LLMs (like Mistral and Minimax).

## Status Report

✅ **SUCCESS: All files are within limits.**

| File | Line Count | Status |
|---|---|---|
| [`mod.rs`](src/apps/mod.rs) | 54 | ✅ OK |
| [`args.rs`](src/args.rs) | 68 | ✅ OK |
| [`caption_overlay.rs`](src/caption_overlay.rs) | 167 | ✅ OK |
| [`atlas.rs`](src/cell_renderer/atlas.rs) | 103 | ✅ OK |
| [`font.rs`](src/cell_renderer/font.rs) | 66 | ✅ OK |
| [`geom.rs`](src/cell_renderer/geom.rs) | 93 | ✅ OK |
| [`gpu_init.rs`](src/cell_renderer/gpu_init.rs) | 237 | ✅ OK |
| [`gpu_render.rs`](src/cell_renderer/gpu_render.rs) | 228 | ✅ OK |
| [`mod.rs`](src/cell_renderer/mod.rs) | 228 | ✅ OK |
| [`pixels.rs`](src/cell_renderer/pixels.rs) | 156 | ✅ OK |
| [`mod.rs`](src/core/logo_block/mod.rs) | 67 | ✅ OK |
| [`alpha_am.rs`](src/core/logo_block/patterns/alpha_am.rs) | 98 | ✅ OK |
| [`alpha_nz.rs`](src/core/logo_block/patterns/alpha_nz.rs) | 98 | ✅ OK |
| [`digits.rs`](src/core/logo_block/patterns/digits.rs) | 77 | ✅ OK |
| [`mod.rs`](src/core/logo_block/patterns/mod.rs) | 26 | ✅ OK |
| [`symbols.rs`](src/core/logo_block/patterns/symbols.rs) | 36 | ✅ OK |
| [`mod.rs`](src/core/mod.rs) | 26 | ✅ OK |
| [`screen_palette.rs`](src/core/screen_palette.rs) | 97 | ✅ OK |
| [`discovery.rs`](src/discovery.rs) | 123 | ✅ OK |
| [`fps_overlay.rs`](src/fps_overlay.rs) | 93 | ✅ OK |
| [`launcher.rs`](src/launcher.rs) | 245 | ✅ OK |
| [`launcher_tests.rs`](src/launcher_tests.rs) | 144 | ✅ OK |
| [`lib.rs`](src/lib.rs) | 34 | ✅ OK |
| [`platform_helpers.rs`](src/platform_helpers.rs) | 72 | ✅ OK |
| [`loading.rs`](src/plugin_session/loading.rs) | 109 | ✅ OK |
| [`mod.rs`](src/plugin_session/mod.rs) | 214 | ✅ OK |
| [`reloading.rs`](src/plugin_session/reloading.rs) | 106 | ✅ OK |
| [`renderer.rs`](src/renderer.rs) | 107 | ✅ OK |
| [`sandbox.rs`](src/sandbox.rs) | 33 | ✅ OK |
| [`terminal_guard.rs`](src/terminal_guard.rs) | 44 | ✅ OK |
| [`linux_proc.rs`](src/toolkit/linux_proc.rs) | 85 | ✅ OK |
| [`linux_queries.rs`](src/toolkit/linux_queries.rs) | 79 | ✅ OK |
| [`mod.rs`](src/toolkit/mod.rs) | 26 | ✅ OK |
| [`platform.rs`](src/toolkit/platform.rs) | 52 | ✅ OK |
| [`mod.rs`](src/toolkit/sys_info/mod.rs) | 143 | ✅ OK |
| [`monitors.rs`](src/toolkit/sys_info/monitors.rs) | 166 | ✅ OK |
| [`theme.rs`](src/toolkit/sys_info/theme.rs) | 60 | ✅ OK |
| [`theme_query.rs`](src/toolkit/theme_query.rs) | 80 | ✅ OK |
| [`trance_runner.rs`](src/trance_runner.rs) | 143 | ✅ OK |
| [`trance_runner_fullscreen.rs`](src/trance_runner_fullscreen.rs) | 219 | ✅ OK |
