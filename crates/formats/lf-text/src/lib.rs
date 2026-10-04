//! Readers for the game's localisation text database (GXT) and the
//! plain-text font and front-end description files.
//!
//! # GXT text database
//!
//! One file per language (for example `american.gxt`). Each file maps short
//! text labels such as `MO_OFF` to their translated strings. Labels are stored
//! only as 32-bit hashes; see [`label_hash`].
//!
//! Layout, all integers little-endian:
//!
//! ```text
//! offset  size  meaning
//! 0       2     format version (observed: 4)
//! 2       2     bits per text unit (observed: 16)
//! 4       4     tag "TABL"
//! 8       4     table-directory size in bytes (always a multiple of 12)
//! 12      N*12  table directory: N entries of
//!                 8 bytes  table name, upper-case ASCII, NUL padded
//!                 4 bytes  file offset of the table's key block
//! ```
//!
//! The first table is always named `MAIN`; the remaining tables are sorted
//! alphabetically. Each table offset points either directly at the key block
//! (`MAIN`) or at an 8-byte copy of the table name followed by the key block.
//!
//! ```text
//! key block:
//!   4 bytes  tag "TKEY"
//!   4 bytes  block size in bytes (multiple of 8)
//!   entries: 4 bytes  byte offset of the string, relative to the first
//!                        string byte (the byte after the TDAT header)
//!              4 bytes  label hash (see [`label_hash`])
//! data block, immediately after the key block:
//!   4 bytes  tag "TDAT"
//!   4 bytes  block size in bytes
//!   bytes    concatenated NUL-terminated strings
//! ```
//!
//! With 16 bits per unit, strings are sequences of little-endian 16-bit
//! codes terminated by a zero unit. The codes are glyph indices for the
//! game's bitmap fonts, not Unicode: western languages only use codes below
//! 256 (matching Windows-1252), Russian additionally uses a page with the
//! high byte `0x80`, and Japanese uses several high-byte pages. Use
//! [`decode_western_lossy`] for a best-effort rendering of western strings;
//! other pages decode to the replacement character.
//!
//! # Text labels and hashing
//!
//! A label such as `MO_OFF` is looked up by hashing it with [`label_hash`]
//! (Jenkins one-at-a-time over the label bytes with ASCII upper-case folded
//! to lower-case) and comparing against the hashes in the key block. Key
//! entries within a table are stored in string order, not hash order.
//!
//! # Font description files (`fonts.dat` and variants)
//!
//! Plain text, one file per script family (western, Japanese, Russian).
//! Sections in square brackets:
//!
//! * `[RESOLUTION]` — the reference resolution the widths are based on.
//! * `[BUTTONS]` — one advance width per controller/keyboard glyph slot, each
//!   followed by a `#` comment naming the slot.
//! * `[RADAR_BLIP]` — the advance width used for blip icons in strings.
//! * Per font, in order: `[FONT_ID]` (numeric id), `[MAP]` … `[/MAP]` (one
//!   glyph code per texture slot), `[MAINFONT]`, `[SUBFONT_1]`, `[SUBFONT_2]`,
//!   `[COMMON_FONT]` (start/end slot ranges into the map), `[PROP]` …
//!   `[/PROP]` (advance widths per slot; may be shorter than the map),
//!   `[UNPROP]`, `[SPACE_BETWEEN_CHARS]` (three spacing values), `[WHITESPACE]`
//!   (width of the space glyph), and on the Japanese font only
//!   `[JAPANESE_SUBFONT_1_WIDTH]` / `[JAPANESE_SUBFONT_2_WIDTH]`.
//!
//! Slot counts differ per font and per file; the parser accepts any length
//! and does not assume the western 208-slot shape.
//!
//! # Front-end description files
//!
//! * `hud.dat` — per display mode (`[HD]`, `[CRT]`) rows of HUD item name,
//!   position, size, colour name and alpha. See [`HudFile`].
//! * `hudColor.dat` — per display mode rows of colour name and RGB triple.
//!   See [`HudColours`].
//! * `frontend.dat`, `frontend_pc.dat`, `frontend_360.dat` — per display mode
//!   rows of layout key and floating-point values. See [`FrontendLayout`].
//! * `radiohud.dat` — texture-container list between `+` markers, then one
//!   row per station: radio id, hash name, monochrome and colour texture
//!   names, visible width and two vertical-alignment modifiers.
//!   See [`RadioHud`].
//! * `frontend_menus.xml` — menu definitions whose options reference GXT
//!   labels by name. See [`MenuFile`].
//!
//! All parsers read from byte slices, use explicit little-endian decoding,
//! are pointer-width independent, and return errors instead of panicking on
//! malformed input.

mod dat;
mod fonts;
mod frontend;
mod gxt;
mod hash;
mod hud;
mod menus;

pub use dat::{DataError, FrontendLayout, LayoutSection, LayoutValue, data_file_lines};
pub use fonts::{ButtonWidth, FontDesc, FontError, FontFile};
pub use frontend::{RadioContainer, RadioHud, RadioHudFile, RadioStation, SimpleStation};
pub use gxt::{GxtEntry, GxtError, GxtFile, GxtTable, decode_western_lossy};
pub use hash::label_hash;
pub use hud::{HudColour, HudColourSection, HudColours, HudFile, HudItem, HudSection, Rgb};
pub use menus::{Menu, MenuError, MenuFile, MenuOption, MenuSection};
