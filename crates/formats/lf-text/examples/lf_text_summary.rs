//! Print a metadata summary of one text/font/front-end file.
//!
//! Usage: `lf_text_summary <path>`. Only names, counts and sizes are printed; GXT
//! string contents are never written out.

use std::path::PathBuf;

use lf_text::{FontFile, FrontendLayout, GxtFile, HudColours, HudFile, MenuFile, RadioHudFile};

fn main() {
    let path: PathBuf = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            eprintln!("usage: lf_text_summary <path>");
            std::process::exit(2);
        });
    let data = std::fs::read(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {}: {e}", path.display());
        std::process::exit(1);
    });
    println!("file: {} ({} bytes)", path.display(), data.len());
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if name.ends_with(".gxt") {
        summarize_gxt(&data);
    } else if name.starts_with("fonts") {
        summarize_fonts(&data);
    } else if name == "hud.dat" {
        summarize_hud(&data);
    } else if name.eq_ignore_ascii_case("hudcolor.dat") {
        summarize_colours(&data);
    } else if name.starts_with("frontend") && name.ends_with(".dat") {
        summarize_layout(&data);
    } else if name == "radiohud.dat" {
        summarize_radio(&data);
    } else if name == "frontend_menus.xml" {
        summarize_menus(&data);
    } else {
        eprintln!("unknown file kind for {name}");
        std::process::exit(2);
    }
}

fn summarize_gxt(data: &[u8]) {
    match GxtFile::parse(data) {
        Ok(file) => {
            println!(
                "kind: GXT version={} bits-per-char={}",
                file.version(),
                file.bits_per_char()
            );
            println!(
                "tables: {} entries: {}",
                file.tables().len(),
                file.entry_count()
            );
            let mut names: Vec<&str> = file.tables().iter().map(|t| t.name.as_str()).collect();
            names.sort_unstable();
            let head: Vec<&&str> = names.iter().take(8).collect();
            println!("table names (first 8 of {}): {head:?}", names.len());
            for table in file.tables().iter().take(3) {
                println!("  {:8} entries: {}", table.name, table.entries.len());
            }
        }
        Err(e) => {
            eprintln!("parse error: {e}");
            std::process::exit(1);
        }
    }
}

fn summarize_fonts(data: &[u8]) {
    match FontFile::parse(data) {
        Ok(file) => {
            println!(
                "kind: fonts.dat resolution={}x{}",
                file.resolution.0, file.resolution.1
            );
            println!(
                "buttons: {} radar_blip: {} fonts: {}",
                file.buttons.len(),
                file.radar_blip,
                file.fonts.len()
            );
            for font in &file.fonts {
                println!(
                    "  id={} slots={} main={:?} sub1={:?} sub2={:?} common={:?} unprop={} spacing={:?} whitespace={}",
                    font.id,
                    font.map.len(),
                    font.main,
                    font.sub1,
                    font.sub2,
                    font.common,
                    font.unprop,
                    font.spacing,
                    font.whitespace
                );
            }
        }
        Err(e) => {
            eprintln!("parse error: {e}");
            std::process::exit(1);
        }
    }
}

fn summarize_hud(data: &[u8]) {
    match HudFile::parse(data) {
        Ok(file) => {
            println!("kind: hud.dat");
            for section in &file.sections {
                println!("  [{}] items: {}", section.name, section.items.len());
            }
        }
        Err(e) => {
            eprintln!("parse error: {e}");
            std::process::exit(1);
        }
    }
}

fn summarize_colours(data: &[u8]) {
    match HudColours::parse(data) {
        Ok(file) => {
            println!("kind: hudColor.dat");
            for section in &file.sections {
                println!("  [{}] colours: {}", section.name, section.colours.len());
            }
        }
        Err(e) => {
            eprintln!("parse error: {e}");
            std::process::exit(1);
        }
    }
}

fn summarize_layout(data: &[u8]) {
    match FrontendLayout::parse(data) {
        Ok(file) => {
            println!("kind: frontend layout");
            for section in &file.sections {
                println!("  [{}] rows: {}", section.name, section.values.len());
            }
        }
        Err(e) => {
            eprintln!("parse error: {e}");
            std::process::exit(1);
        }
    }
}

fn summarize_radio(data: &[u8]) {
    match RadioHudFile::parse(data) {
        Ok(RadioHudFile::Full(file)) => {
            println!("kind: radiohud.dat (full)");
            println!(
                "containers: {} stations: {}",
                file.containers.len(),
                file.stations.len()
            );
        }
        Ok(RadioHudFile::Simple(rows)) => {
            println!("kind: radiohud.dat (simple)");
            println!("stations: {}", rows.len());
        }
        Err(e) => {
            eprintln!("parse error: {e}");
            std::process::exit(1);
        }
    }
}

fn summarize_menus(data: &[u8]) {
    match MenuFile::parse(data) {
        Ok(file) => {
            println!("kind: frontend_menus.xml version={}", file.version);
            for section in &file.sections {
                let options: usize = section.menus.iter().map(|m| m.options.len()).sum();
                println!(
                    "  {} menus: {} options: {options}",
                    section.name,
                    section.menus.len(),
                );
            }
            println!("labels referenced: {}", file.iter_labels().count());
        }
        Err(e) => {
            eprintln!("parse error: {e}");
            std::process::exit(1);
        }
    }
}
