//! Summarise one gameplay data file: parser used, record counts, sizes.
//!
//! Usage: `lf_gamedata_summarize <path-to-data-file>`.
//! Prints to stdout only; never writes anything.

use lf_gamedata::route::{Parsed, parse_file};
use std::path::PathBuf;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: lf_gamedata_summarize <path-to-data-file>");
        std::process::exit(2);
    });
    let bytes = std::fs::read(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {path}: {e}");
        std::process::exit(1);
    });
    let name: PathBuf = path.into();
    let display = name.to_string_lossy().replace('\\', "/");
    println!("file: {display}");
    println!("size: {} bytes", bytes.len());
    match parse_file(&display, &bytes) {
        Ok(parsed) => {
            println!("parser: {}", parsed.parser());
            for (table, count) in parsed.counts() {
                println!("  {table}: {count}");
            }
            // A few names for eyeballing, per format.
            match &parsed {
                Parsed::Handling(h) => {
                    for c in h.cars.iter().take(5) {
                        println!("  car: {}", c.name);
                    }
                }
                Parsed::Ide(o) => {
                    for c in o.cars.iter().take(5) {
                        println!("  car: {} ({})", c.model, c.vehicle_type);
                    }
                    for (p, healed) in o.peds.iter().take(5) {
                        println!("  ped: {} (healed={healed})", p.model);
                    }
                }
                Parsed::WeaponInfo(w) => {
                    for w in w.weapons.iter().take(8) {
                        println!("  weapon: {}", w.weapon_type);
                    }
                }
                Parsed::Timecyc(w) => {
                    for w in w {
                        println!("  weather: {} ({} slots)", w.name, w.slots.len());
                    }
                }
                Parsed::Popcycle(z) => {
                    for z in z.iter().take(5) {
                        println!("  zone: {}", z.name);
                    }
                }
                _ => {}
            }
        }
        Err(e) => {
            println!("error: {e}");
            std::process::exit(1);
        }
    }
}
