// SPDX-License-Identifier: MIT
//! GPU cell-renderer: orchestrator + phase helpers.
//!
//! - [`render::render`] is the public entry point. It's a thin
//!   orchestrator over the phase helpers in [`phases`].
//! - [`phases`] contains one method per stage of the per-frame
//!   pipeline (target texture, atlas, bind group, uniforms, render
//!   pass, staging copy).

mod phases;
mod render;
