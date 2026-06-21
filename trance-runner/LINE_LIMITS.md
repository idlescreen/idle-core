# Codebase File Line Limits

This project enforces a range of **25 to 250 lines** per source file to ensure readability and compatibility with smaller LLMs (like Mistral and Minimax).

## Status Report

✅ **SUCCESS: All files are within limits.**

| File | Line Count | Status |
|---|---|---|
| [`src/apps/mod.rs`](src/apps/mod.rs) | 58 | ✅ OK |
| [`src/args.rs`](src/args.rs) | 68 | ✅ OK |
| [`src/caption_overlay.rs`](src/caption_overlay.rs) | 163 | ✅ OK |
| [`src/cell_renderer/mod.rs`](src/cell_renderer/mod.rs) | 230 | ✅ OK |
| [`src/cell_renderer/pixels.rs`](src/cell_renderer/pixels.rs) | 121 | ✅ OK |
| [`src/core/logo_block/mod.rs`](src/core/logo_block/mod.rs) | 66 | ✅ OK |
| [`src/core/logo_block/patterns/alpha_am.rs`](src/core/logo_block/patterns/alpha_am.rs) | 98 | ✅ OK |
| [`src/core/logo_block/patterns/alpha_nz.rs`](src/core/logo_block/patterns/alpha_nz.rs) | 98 | ✅ OK |
| [`src/core/logo_block/patterns/digits.rs`](src/core/logo_block/patterns/digits.rs) | 77 | ✅ OK |
| [`src/core/logo_block/patterns/mod.rs`](src/core/logo_block/patterns/mod.rs) | 26 | ✅ OK |
| [`src/core/logo_block/patterns/symbols.rs`](src/core/logo_block/patterns/symbols.rs) | 36 | ✅ OK |
| [`src/core/mod.rs`](src/core/mod.rs) | 26 | ✅ OK |
| [`src/core/screen_palette.rs`](src/core/screen_palette.rs) | 89 | ✅ OK |
| [`src/discovery.rs`](src/discovery.rs) | 101 | ✅ OK |
| [`src/fps_overlay.rs`](src/fps_overlay.rs) | 87 | ✅ OK |
| [`src/launcher.rs`](src/launcher.rs) | 161 | ✅ OK |
| [`src/launcher_tests.rs`](src/launcher_tests.rs) | 45 | ✅ OK |
| [`src/lib.rs`](src/lib.rs) | 28 | ✅ OK |
| [`src/platform_helpers.rs`](src/platform_helpers.rs) | 73 | ✅ OK |
| [`src/plugin_session.rs`](src/plugin_session.rs) | 250 | ✅ OK |
| [`src/renderer.rs`](src/renderer.rs) | 106 | ✅ OK |
| [`src/terminal_guard.rs`](src/terminal_guard.rs) | 44 | ✅ OK |
| [`src/toolkit/linux_proc.rs`](src/toolkit/linux_proc.rs) | 85 | ✅ OK |
| [`src/toolkit/linux_queries.rs`](src/toolkit/linux_queries.rs) | 80 | ✅ OK |
| [`src/toolkit/mod.rs`](src/toolkit/mod.rs) | 26 | ✅ OK |
| [`src/toolkit/platform.rs`](src/toolkit/platform.rs) | 52 | ✅ OK |
| [`src/toolkit/sys_info/mod.rs`](src/toolkit/sys_info/mod.rs) | 143 | ✅ OK |
| [`src/toolkit/sys_info/monitors.rs`](src/toolkit/sys_info/monitors.rs) | 165 | ✅ OK |
| [`src/toolkit/sys_info/theme.rs`](src/toolkit/sys_info/theme.rs) | 61 | ✅ OK |
| [`src/toolkit/theme_query.rs`](src/toolkit/theme_query.rs) | 77 | ✅ OK |
| [`src/trance_runner.rs`](src/trance_runner.rs) | 246 | ✅ OK |
