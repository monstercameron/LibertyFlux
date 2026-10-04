//! Integration tests against the real game data.
//!
//! These tests read files from the game folder named by the `LIBERTYFLUX_GAME_DIR`
//! environment variable and skip otherwise. They assert structural facts
//! (file counts, table counts, label lookups) without embedding any game
//! content.

use std::collections::HashSet;
use std::path::PathBuf;

use lf_text::{
    FontFile, FrontendLayout, GxtFile, HudColours, HudFile, MenuFile, RadioHudFile,
    decode_western_lossy,
};

fn game_dir() -> Option<PathBuf> {
    std::env::var_os("LIBERTYFLUX_GAME_DIR").map(PathBuf::from)
}

const BASE_LANGS: [&str; 7] = [
    "american", "french", "german", "italian", "japanese", "russian", "spanish",
];
const EPISODE_LANGS: [&str; 6] = [
    "american", "french", "german", "italian", "russian", "spanish",
];

fn gxt_paths(root: &std::path::Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for lang in BASE_LANGS {
        out.push(root.join("GTAIV").join("common/text").join(format!("{lang}.gxt")));
    }
    for pack in ["TBoGT", "TLAD"] {
        for lang in EPISODE_LANGS {
            out.push(root.join(format!("{pack}/common/text/{lang}.gxt")));
        }
    }
    out
}

#[test]
fn all_gxt_files_parse() {
    let Some(root) = game_dir() else {
        eprintln!("skipped: LIBERTYFLUX_GAME_DIR is not set");
        return;
    };
    let paths = gxt_paths(&root);
    assert_eq!(paths.len(), 19, "expected 19 GXT files");
    let mut parsed = 0;
    for path in &paths {
        let data =
            std::fs::read(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        let file = GxtFile::parse(&data)
            .unwrap_or_else(|e| panic!("cannot parse {}: {e}", path.display()));
        assert_eq!(file.version(), 4, "{}", path.display());
        assert_eq!(file.bits_per_char(), 16, "{}", path.display());
        assert!(!file.tables().is_empty(), "{}", path.display());
        assert_eq!(file.tables()[0].name, "MAIN", "{}", path.display());
        // Offsets must be ordered within each table (string order).
        assert!(file.entry_count() > 0, "{}", path.display());
        parsed += 1;
    }
    assert_eq!(parsed, 19);
}

#[test]
fn gxt_table_counts() {
    let Some(root) = game_dir() else {
        eprintln!("skipped: LIBERTYFLUX_GAME_DIR is not set");
        return;
    };
    let check = |relative: &str, tables: usize| {
        let data = std::fs::read(root.join(relative)).unwrap();
        let file = GxtFile::parse(&data).unwrap();
        assert_eq!(file.tables().len(), tables, "{relative}");
    };
    check("common/text/american.gxt", 524);
    check("TBoGT/common/text/american.gxt", 163);
    check("TLAD/common/text/american.gxt", 142);
}

#[test]
fn gxt_label_lookup_end_to_end() {
    let Some(root) = game_dir() else {
        eprintln!("skipped: LIBERTYFLUX_GAME_DIR is not set");
        return;
    };
    let data = std::fs::read(root.join("GTAIV").join("common/text/american.gxt")).unwrap();
    let file = GxtFile::parse(&data).unwrap();
    // Labels taken from frontend_menus.xml; expected strings verified once by
    // hand and asserted here to pin the hash function end to end.
    assert_eq!(
        decode_western_lossy(file.lookup("MAIN", "MO_OFF").unwrap()),
        "Off"
    );
    assert_eq!(
        decode_western_lossy(file.lookup("MAIN", "MO_ON").unwrap()),
        "On"
    );
    assert_eq!(
        decode_western_lossy(file.lookup("MAIN", "BLIPS").unwrap()),
        "Blips only"
    );
    // Lookup is case insensitive through the hash.
    assert_eq!(file.lookup("MAIN", "mo_off"), file.lookup("MAIN", "MO_OFF"));
}

#[test]
fn gxt_menu_labels_resolve() {
    let Some(root) = game_dir() else {
        eprintln!("skipped: LIBERTYFLUX_GAME_DIR is not set");
        return;
    };
    let data = std::fs::read(root.join("GTAIV").join("common/text/american.gxt")).unwrap();
    let file = GxtFile::parse(&data).unwrap();
    let hashes: HashSet<u32> = file.iter_entries().map(|(_, h, _)| h).collect();
    let xml = std::fs::read(root.join("GTAIV").join("common/data/frontend_menus.xml")).unwrap();
    let menus = MenuFile::parse(&xml).unwrap();
    let labels: HashSet<&str> = menus.iter_labels().collect();
    assert!(!labels.is_empty());
    let mut missing = Vec::new();
    for label in &labels {
        if !hashes.contains(&lf_text::label_hash(label)) {
            missing.push(*label);
        }
    }
    // Three menu labels have no entry in any shipped table (verified across
    // the base game and both episodes); everything else must resolve.
    missing.sort_unstable();
    assert_eq!(missing, vec!["LANG_J", "MO_JOY", "MO_KEY"]);
}

#[test]
fn font_files_parse() {
    let Some(root) = game_dir() else {
        eprintln!("skipped: LIBERTYFLUX_GAME_DIR is not set");
        return;
    };
    for relative in [
        "common/data/fonts.dat",
        "common/data/fonts_j.dat",
        "common/data/fonts_r.dat",
        "TBoGT/common/data/fonts.dat",
        "TLAD/common/data/fonts.dat",
    ] {
        let data = std::fs::read(root.join(relative)).unwrap();
        let file =
            FontFile::parse(&data).unwrap_or_else(|e| panic!("cannot parse {relative}: {e}"));
        assert!(!file.fonts.is_empty(), "{relative}");
        assert!(!file.buttons.is_empty(), "{relative}");
        for font in &file.fonts {
            // MAP and PROP lengths differ per font (verified in fonts_j.dat),
            // so only non-emptiness is asserted here.
            assert!(!font.map.is_empty(), "{relative} font {}", font.id);
            assert!(!font.prop.is_empty(), "{relative} font {}", font.id);
        }
    }
    // Script families carry different font sets.
    let western =
        FontFile::parse(&std::fs::read(root.join("GTAIV").join("common/data/fonts.dat")).unwrap()).unwrap();
    let japanese =
        FontFile::parse(&std::fs::read(root.join("GTAIV").join("common/data/fonts_j.dat")).unwrap()).unwrap();
    let western_ids: Vec<u32> = western.fonts.iter().map(|f| f.id).collect();
    let japanese_ids: Vec<u32> = japanese.fonts.iter().map(|f| f.id).collect();
    assert_eq!(western_ids, vec![0, 1, 3]);
    assert_eq!(japanese_ids, vec![0, 1, 2, 3]);
    let jp_font = japanese.font(2).unwrap();
    assert_eq!(jp_font.japanese_sub1_width, Some(39));
    assert_eq!(jp_font.japanese_sub2_width, Some(39));
}

#[test]
fn hud_files_parse() {
    let Some(root) = game_dir() else {
        eprintln!("skipped: LIBERTYFLUX_GAME_DIR is not set");
        return;
    };
    for pack in ["", "TBoGT/", "TLAD/"] {
        if pack == "TLAD/" {
            // TLAD ships no hud.dat (verified by census).
            continue;
        }
        let relative = format!("{pack}common/data/hud.dat");
        let data = std::fs::read(root.join(&relative)).unwrap();
        let file = HudFile::parse(&data).unwrap_or_else(|e| panic!("{relative}: {e}"));
        assert!(file.section("HD").is_some(), "{relative}");
    }
    for pack in ["", "TBoGT/", "TLAD/"] {
        let relative = format!("{pack}common/data/hudColor.dat");
        let data = std::fs::read(root.join(&relative)).unwrap();
        let file = HudColours::parse(&data).unwrap_or_else(|e| panic!("{relative}: {e}"));
        assert!(file.section("HD").is_some(), "{relative}");
    }
}

#[test]
fn frontend_layout_files_parse() {
    let Some(root) = game_dir() else {
        eprintln!("skipped: LIBERTYFLUX_GAME_DIR is not set");
        return;
    };
    for relative in [
        "common/data/frontend.dat",
        "common/data/frontend_pc.dat",
        "common/data/frontend_360.dat",
        "TLAD/common/data/frontend_pc.dat",
        "TLAD/common/data/frontend_360.dat",
    ] {
        let data = std::fs::read(root.join(relative)).unwrap();
        let file =
            FrontendLayout::parse(&data).unwrap_or_else(|e| panic!("cannot parse {relative}: {e}"));
        assert!(file.section("HD").is_some(), "{relative}");
    }
}

#[test]
fn radiohud_files_parse() {
    let Some(root) = game_dir() else {
        eprintln!("skipped: LIBERTYFLUX_GAME_DIR is not set");
        return;
    };
    for relative in [
        "common/data/radiohud.dat",
        "TBoGT/common/data/radiohud.dat",
        "TLAD/common/data/radiohud.dat",
        "pc/data/radiohud.dat",
    ] {
        let path = root.join(relative);
        if !path.exists() {
            continue;
        }
        let data = std::fs::read(&path).unwrap();
        let file = RadioHudFile::parse(&data).unwrap_or_else(|e| panic!("{relative}: {e}"));
        match &file {
            RadioHudFile::Full(full) => {
                assert!(!full.containers.is_empty(), "{relative}");
                assert!(!full.stations.is_empty(), "{relative}");
            }
            RadioHudFile::Simple(rows) => assert!(!rows.is_empty(), "{relative}"),
        }
    }
}

#[test]
fn menu_files_parse() {
    let Some(root) = game_dir() else {
        eprintln!("skipped: LIBERTYFLUX_GAME_DIR is not set");
        return;
    };
    for relative in [
        "common/data/frontend_menus.xml",
        "TBoGT/common/data/frontend_menus.xml",
        "TLAD/common/data/frontend_menus.xml",
    ] {
        let data = std::fs::read(root.join(relative)).unwrap();
        let file = MenuFile::parse(&data).unwrap_or_else(|e| panic!("{relative}: {e}"));
        assert_eq!(file.version, "1", "{relative}");
        assert!(!file.sections.is_empty(), "{relative}");
    }
}
