//! `cutscene`: `.cut` cutscene descriptions (`lf-cutscene`).
//!
//! - `summarize <file.cut>`: groups, sections, timing, names (the former
//!   `lf_cutscene_summarize` example, same output).
//! - `list <file.cut>`: one line per section: group, section, animation,
//!   audio, duration.
//! - `catalog`: JSON catalog of every cutscene in the game folder named by
//!   `LIBERTYFLUX_GAME_DIR` (the former `lf_cutscene_catalog` example).
//!   Reads the folder only; the archive key stays in memory.

use std::collections::HashSet;
use std::path::PathBuf;

use lf_cutscene::catalog;
use lf_cutscene::cut::CutsceneFile;

use crate::cli::{CliError, CliResult, Io, load_key, one_path, read_file};

/// Subcommands this format offers.
pub const COMMANDS: &[&str] = &["summarize", "list", "catalog"];

const USAGE: &str = "lf-inspect cutscene summarize|list <file.cut> | lf-inspect cutscene catalog";

/// Run one `cutscene` subcommand.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when the file
/// cannot be read or parsed.
pub fn run(cmd: &str, args: &[String], io: &mut Io) -> CliResult {
    match cmd {
        "catalog" => catalog_cmd(io),
        "summarize" | "list" => {
            let path = one_path(args, USAGE)?;
            let bytes = read_file(path)?;
            let file = CutsceneFile::parse(&bytes).map_err(|e| CliError::failure(e.to_string()))?;
            if cmd == "list" {
                for (gi, g) in file.groups.iter().enumerate() {
                    for (si, s) in g.sections.iter().enumerate() {
                        writeln!(
                            io.out,
                            "{gi} {si} anim={} audio={} ms={:.0}",
                            s.anims.join(","),
                            s.audios.join(","),
                            s.durations_ms.iter().sum::<f32>()
                        )?;
                    }
                }
                return Ok(());
            }
            summarize(path, &bytes, &file, io)
        }
        _ => Err(CliError::usage(format!("usage: {USAGE}"))),
    }
}

fn summarize(path: &str, bytes: &[u8], file: &CutsceneFile, io: &mut Io) -> CliResult {
    let o = &mut *io.out;
    writeln!(o, "file: {path} ({} bytes)", bytes.len())?;
    writeln!(o, "cutscenes: {}", file.groups.len())?;
    writeln!(o, "trailing slack: {} bytes", file.trailing_slack_len)?;
    for (gi, g) in file.groups.iter().enumerate() {
        let anims: HashSet<&str> = g
            .sections
            .iter()
            .flat_map(|s| s.anims.iter().map(String::as_str))
            .collect();
        let mut anims: Vec<&str> = anims.into_iter().collect();
        anims.sort_unstable();
        let audios: HashSet<&str> = g
            .sections
            .iter()
            .flat_map(|s| s.audios.iter().map(String::as_str))
            .collect();
        let mut audios: Vec<&str> = audios.into_iter().collect();
        audios.sort_unstable();
        writeln!(
            o,
            "group {gi}: {} sections, {:.0} ms, {} models, {} subtitles",
            g.sections.len(),
            g.duration_ms(),
            g.model_count(),
            g.texts.len()
        )?;
        writeln!(o, "  header frames: {:?}", g.header_frames)?;
        writeln!(o, "  anims: {anims:?}")?;
        writeln!(o, "  audios: {audios:?}")?;
        writeln!(o, "  flags: {:?}", g.flags)?;
    }
    for w in &file.warnings {
        writeln!(o, "warning: line {} {:?}: {}", w.line, w.kind, w.message)?;
    }
    Ok(())
}

fn catalog_cmd(io: &mut Io) -> CliResult {
    let dir: PathBuf = std::env::var_os("LIBERTYFLUX_GAME_DIR")
        .map(PathBuf::from)
        .ok_or_else(|| CliError::failure("set LIBERTYFLUX_GAME_DIR to the game folder"))?;
    let exe = dir.join("GTAIV").join("GTAIV.exe");
    let key = load_key(&exe.to_string_lossy())?;
    let mut parsed = Vec::new();
    for path in catalog::find_archives(&dir, "cuts.img") {
        parsed.extend(
            catalog::parse_cuts_in_archive(&path, &key)
                .map_err(|e| CliError::failure(e.to_string()))?,
        );
    }
    let rows = catalog::catalog_rows(&parsed);
    write!(io.out, "{}", catalog::catalog_json(&rows))?;
    Ok(())
}
