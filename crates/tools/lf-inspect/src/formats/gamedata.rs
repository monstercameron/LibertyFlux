//! `gamedata`: gameplay data files routed by file name (`lf-gamedata`).
//!
//! - `summarize <path-to-data-file>`: parser used, record counts and a few
//!   names (the former `lf_gamedata_summarize` example, same output).
//! - `dump <path-to-data-file>`: parser and counts as JSON-ish text.

use lf_gamedata::route::{Parsed, parse_file};

use crate::cli::{CliError, CliResult, EXIT_FAILURE, Io, json_str, one_path, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize", "dump"];

const USAGE: &str = "lf-inspect gamedata summarize|dump <path-to-data-file>";

/// Run one `gamedata` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when the file
/// cannot be read or parsed.
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    if !COMMANDS.contains(&cmd) {
        return Err(CliError::usage(format!("usage: {USAGE}")));
    }
    let path = one_path(args, USAGE)?;
    let bytes = read_file(path)?;
    let display = path.replace('\\', "/");
    if cmd == "dump" {
        let parsed =
            parse_file(&display, &bytes).map_err(|e| CliError::failure(format!("error: {e}")))?;
        let counts: Vec<String> = parsed
            .counts()
            .iter()
            .map(|(t, n)| format!("{}: {n}", json_str(t)))
            .collect();
        writeln!(
            io.out,
            "{{\"file\": {}, \"parser\": {}, \"counts\": {{{}}}}}",
            json_str(&display),
            json_str(parsed.parser()),
            counts.join(", ")
        )?;
        return Ok(());
    }
    let o = &mut *io.out;
    writeln!(o, "file: {display}")?;
    writeln!(o, "size: {} bytes", bytes.len())?;
    match parse_file(&display, &bytes) {
        Ok(parsed) => {
            writeln!(o, "parser: {}", parsed.parser())?;
            for (table, count) in parsed.counts() {
                writeln!(o, "  {table}: {count}")?;
            }
            // A few names for eyeballing, per format.
            match &parsed {
                Parsed::Handling(h) => {
                    for c in h.cars.iter().take(5) {
                        writeln!(o, "  car: {}", c.name)?;
                    }
                }
                Parsed::Ide(ide) => {
                    for c in ide.cars.iter().take(5) {
                        writeln!(o, "  car: {} ({})", c.model, c.vehicle_type)?;
                    }
                    for (p, healed) in ide.peds.iter().take(5) {
                        writeln!(o, "  ped: {} (healed={healed})", p.model)?;
                    }
                }
                Parsed::WeaponInfo(w) => {
                    for w in w.weapons.iter().take(8) {
                        writeln!(o, "  weapon: {}", w.weapon_type)?;
                    }
                }
                Parsed::Timecyc(w) => {
                    for w in w {
                        writeln!(o, "  weather: {} ({} slots)", w.name, w.slots.len())?;
                    }
                }
                Parsed::Popcycle(z) => {
                    for z in z.iter().take(5) {
                        writeln!(o, "  zone: {}", z.name)?;
                    }
                }
                _ => {}
            }
            Ok(())
        }
        Err(e) => {
            writeln!(o, "error: {e}")?;
            Err(CliError::silent(EXIT_FAILURE))
        }
    }
}
