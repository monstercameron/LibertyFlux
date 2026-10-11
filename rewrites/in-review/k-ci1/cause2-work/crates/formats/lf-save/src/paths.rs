//! Where save files live and what they are called.
//!
//! Slot files are named `SGTA4xx`. The executable references sixteen names,
//! `SGTA400` to `SGTA415`; public documentation describes twelve manual
//! slots plus per-title autosave slots. Complete Edition saves live under
//! the user's documents folder in a per-account `Profiles` directory; older
//! Games for Windows Live saves lived under the local application-data
//! folder. All locations here are derived from environment variables only,
//! so this module stays platform independent (and simply finds nothing
//! where saves cannot exist).

use std::path::PathBuf;

/// First slot number referenced by the executable.
pub const FIRST_SLOT: u32 = 0;

/// Last slot number referenced by the executable.
pub const LAST_SLOT: u32 = 15;

/// Slot file name for a zero-based slot number: `SGTA400` to `SGTA415`.
///
/// Returns `None` past the names the executable references.
#[must_use]
pub fn slot_name(slot: u32) -> Option<String> {
    if (FIRST_SLOT..=LAST_SLOT).contains(&slot) {
        Some(format!("SGTA4{slot:02}"))
    } else {
        None
    }
}

/// Directories that may hold save files and currently exist.
///
/// Candidates, in order:
/// - the Complete Edition `Profiles` folder under the user's documents
///   (`%USERPROFILE%\Documents\Rockstar Games\GTA IV\Profiles`),
/// - the legacy Games for Windows Live save folder under local
///   application data (`%LOCALAPPDATA%\Rockstar Games\GTA IV\savegames`).
///
/// Only directories that exist are returned. Paths come from environment
/// variables; nothing is created or written.
#[must_use]
pub fn save_dirs() -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(profile) = std::env::var("USERPROFILE") {
        let mut p = PathBuf::from(profile);
        p.push("Documents");
        p.push("Rockstar Games");
        p.push("GTA IV");
        p.push("Profiles");
        if p.is_dir() {
            out.push(p);
        }
    }
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        let mut p = PathBuf::from(local);
        p.push("Rockstar Games");
        p.push("GTA IV");
        p.push("savegames");
        if p.is_dir() {
            out.push(p);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_names_cover_the_exe_range() {
        assert_eq!(slot_name(0).as_deref(), Some("SGTA400"));
        assert_eq!(slot_name(11).as_deref(), Some("SGTA411"));
        assert_eq!(slot_name(12).as_deref(), Some("SGTA412"));
        assert_eq!(slot_name(15).as_deref(), Some("SGTA415"));
        assert_eq!(slot_name(16), None);
    }

    #[test]
    fn save_dirs_only_returns_dirs() {
        // Whatever the machine holds, every entry must be a directory.
        for dir in save_dirs() {
            assert!(dir.is_dir());
        }
    }
}
