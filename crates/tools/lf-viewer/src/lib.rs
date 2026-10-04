//! `lf-viewer`: converts a model and its textures from the user's own game
//! files into standard files a viewer opens: glTF 2.0 (`.gltf` JSON plus a
//! `.bin` buffer) for meshes and PNG for textures.
//!
//! README for future lanes:
//! - This is a command-line tool, not a GUI (a window needs a download that
//!   is not approved). Run `lf-viewer --help` for usage. Open the output in
//!   any glTF viewer.
//! - It is built on the format readers (`lf-model`, `lf-texture`) and the
//!   shared maths types (`lf-math`). Everything else is written here with
//!   no dependencies: a PNG encoder ([`png`], zlib stored blocks, CRC-32,
//!   Adler-32), a JSON writer ([`json`]) and a glTF writer ([`gltf`]).
//!   Texture pixels come from `lf-texture`, which already decodes DXT1,
//!   DXT3, DXT5, A8R8G8B8 and L8 to RGBA, so no block decoder lives here.
//! - [`convert`] documents exactly what is carried over and which
//!   assumptions (axes, winding, which texture is the base colour) are
//!   Inferred or Unknown.
//! - It reads only the paths the user passes and writes only inside the
//!   output folder the user passes, never overwriting without `--force`.
//! - Two more commands build on the same readers: `info` ([`info`]) prints
//!   what a model or texture dictionary holds, as text or `--json`, and
//!   writes nothing; `batch` ([`batch`]) converts every model and texture
//!   dictionary in a folder, each into its own folder under the output
//!   folder, and ends with a summary of what was converted and skipped.
//!
//! **Outputs are derived from the user's own copy of the game and must never
//! be committed, uploaded or shared** (AGENTS.md: no game files or assets
//! are committed, in any form). Keep them outside the repository, or under
//! the ignored `.artifacts/` folder.
//!
//! Testing: everything is tested on hand-built inputs (the same way the
//! format crates build fixtures). Converting real files is a local check on
//! a machine with the game.

pub mod batch;
pub mod cli;
pub mod convert;
pub mod gltf;
pub mod info;
pub mod json;
pub mod png;

// Tests compare exactly representable floats and index small fixtures.
#[cfg(test)]
#[allow(clippy::float_cmp, clippy::cast_possible_truncation)]
mod tests;
