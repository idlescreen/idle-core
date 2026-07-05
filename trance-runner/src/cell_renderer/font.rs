// SPDX-License-Identifier: MIT

use std::fs;
use std::path::Path;

pub const FONT_CANDIDATES: &[&str] = &[
    "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
    "/usr/share/fonts/truetype/ubuntu/UbuntuMono-R.ttf",
    "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf",
];

/// Returns the first installed monospace font used for cell rasterization.
pub fn resolve_font_path() -> Option<&'static str> {
    FONT_CANDIDATES
        .iter()
        .find(|path| Path::new(path).is_file())
        .copied()
}

/// Whether a supported monospace font is installed on the system.
pub fn font_available() -> bool {
    resolve_font_path().is_some()
}

pub(crate) fn load_monospace_font() -> Result<Vec<u8>, String> {
    let path = resolve_font_path().ok_or_else(|| {
        "no monospace font found; install the fonts-dejavu-core package".to_string()
    })?;
    fs::read(path).map_err(|error| format!("failed to read {path}: {error}"))
}
