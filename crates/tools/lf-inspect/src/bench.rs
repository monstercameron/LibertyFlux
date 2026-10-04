//! `lf-inspect bench`: parse throughput with `std::time` only.
//!
//! - `bench <format> <file> [--iters N]`: read the file once, then parse
//!   it repeatedly from memory. Without `--iters` it runs at least
//!   [`MIN_ITERS`] times and until [`TIME_BUDGET`] has passed (at most
//!   [`MAX_ITERS`] runs).
//! - `bench synthetic [--iters N]`: the same measurement over fixtures
//!   generated in memory (see [`crate::synth`]), one line per format; with
//!   the default of [`SYNTHETIC_ITERS`] runs it finishes in seconds.
//!
//! Each line reports the input size, the run count, the fastest, median
//! and mean parse time, the throughput of the median run, and the number
//! of items the parse produced (entries, records, instructions: whatever
//! the format counts), which also keeps the work from being optimised away.

use std::io::Cursor;
use std::path::Path;
use std::time::{Duration, Instant};

use crate::cli::{CliError, CliResult, Io, has_ext, read_file};
use crate::synth;

/// Fewest timed runs for a file benchmark.
pub const MIN_ITERS: usize = 3;
/// Most timed runs for a file benchmark without `--iters`.
pub const MAX_ITERS: usize = 1000;
/// Wall-clock budget for a file benchmark without `--iters`.
pub const TIME_BUDGET: Duration = Duration::from_secs(2);
/// Default runs per synthetic fixture.
pub const SYNTHETIC_ITERS: usize = 5;

const USAGE: &str =
    "lf-inspect bench <format> <file> [--iters N] | lf-inspect bench synthetic [--iters N]";

/// A parser under test: file name and bytes in, item count out.
pub type ParseFn = fn(&str, &[u8]) -> Result<usize, String>;

/// Timing summary of one benchmark.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stats {
    /// Timed runs.
    pub iters: usize,
    /// Fastest run.
    pub min: Duration,
    /// Median run.
    pub median: Duration,
    /// Mean run.
    pub mean: Duration,
    /// Items the parse produced.
    pub items: usize,
}

/// Run `parse` on `bytes` (`iters` times, or time-boxed when `None`).
///
/// # Errors
///
/// Returns the parser's message when the input does not parse.
pub fn measure(
    parse: ParseFn,
    name: &str,
    bytes: &[u8],
    iters: Option<usize>,
) -> Result<Stats, String> {
    // One untimed run checks the input and warms caches.
    let items = parse(name, bytes)?;
    let mut times = Vec::new();
    let start = Instant::now();
    loop {
        let t = Instant::now();
        let n = parse(name, bytes)?;
        times.push(t.elapsed());
        debug_assert_eq!(n, items);
        let done = match iters {
            Some(want) => times.len() >= want.max(1),
            None => {
                times.len() >= MAX_ITERS
                    || (times.len() >= MIN_ITERS && start.elapsed() >= TIME_BUDGET)
            }
        };
        if done {
            break;
        }
    }
    times.sort_unstable();
    let total: Duration = times.iter().sum();
    let count = u32::try_from(times.len()).unwrap_or(u32::MAX);
    Ok(Stats {
        iters: times.len(),
        min: times[0],
        median: times[times.len() / 2],
        mean: total / count,
        items,
    })
}

/// Format one result line.
#[must_use]
pub fn line(label: &str, bytes: usize, s: &Stats) -> String {
    let ms = |d: Duration| d.as_secs_f64() * 1000.0;
    #[allow(clippy::cast_precision_loss)] // sizes are far below 2^52 bytes
    let mb = bytes as f64 / (1024.0 * 1024.0);
    let secs = s.median.as_secs_f64();
    let rate = if secs > 0.0 { mb / secs } else { f64::INFINITY };
    format!(
        "{label:<14} {bytes:>10} B  iters {:>4}  min {:>9.3} ms  median {:>9.3} ms  mean {:>9.3} ms  {rate:>9.1} MB/s  items {}",
        s.iters,
        ms(s.min),
        ms(s.median),
        ms(s.mean),
        s.items
    )
}

/// Run `lf-inspect bench ...`.
///
/// # Errors
///
/// Returns a usage error for bad arguments and a failure when the input
/// cannot be read or parsed.
pub fn run(args: &[String], io: &mut Io) -> CliResult {
    let usage = || CliError::usage(format!("usage: {USAGE}"));
    let mut iters = None;
    let mut rest = Vec::new();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--iters" {
            let n = it
                .next()
                .and_then(|s| s.parse::<usize>().ok())
                .ok_or_else(usage)?;
            iters = Some(n);
        } else {
            rest.push(a.as_str());
        }
    }
    match rest.as_slice() {
        ["synthetic"] => synthetic(iters.unwrap_or(SYNTHETIC_ITERS), io),
        [format, file] => {
            let parse = parser_for(format)
                .ok_or_else(|| CliError::usage(format!("no benchmark for format {format}")))?;
            let bytes = read_file(file)?;
            let name = Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(file);
            let stats = measure(parse, name, &bytes, iters)
                .map_err(|e| CliError::failure(format!("{file}: {e}")))?;
            writeln!(io.out, "{}", line(format, bytes.len(), &stats))?;
            Ok(())
        }
        _ => Err(usage()),
    }
}

/// Generate every synthetic fixture and time it.
fn synthetic(iters: usize, io: &mut Io) -> CliResult {
    let cases: Vec<(&str, &str, Vec<u8>, ParseFn)> = vec![
        (
            "resource",
            "synthetic.rsc",
            synth::resource(256),
            parse_resource,
        ),
        (
            "archive-rpf",
            "synthetic.rpf",
            synth::rpf2(4000),
            parse_archive,
        ),
        (
            "archive-img",
            "synthetic.img",
            synth::img(4000),
            parse_archive,
        ),
        ("text-gxt", "synthetic.gxt", synth::gxt(20_000), parse_text),
        (
            "mapdata-wpl",
            "synthetic.wpl",
            synth::wpl(20_000),
            parse_mapdata,
        ),
        ("nav-nod", "synthetic.nod", synth::nod(5000, 4), parse_nav),
        ("sco", "synthetic.sco", synth::sco(100_000), parse_sco),
        (
            "gamedata-ide",
            "vehicles.ide",
            synth::ide_text(5000),
            parse_gamedata,
        ),
        ("save", "SGTA400", synth::save(32, 8192), parse_save),
    ];
    for (label, name, bytes, parse) in cases {
        let stats = measure(parse, name, &bytes, Some(iters))
            .map_err(|e| CliError::failure(format!("synthetic {label}: {e}")))?;
        writeln!(io.out, "{}", line(label, bytes.len(), &stats))?;
    }
    Ok(())
}

/// The benchmark parser for a format name.
#[must_use]
pub fn parser_for(format: &str) -> Option<ParseFn> {
    Some(match format {
        "archive" => parse_archive,
        "resource" => parse_resource,
        "texture" => parse_texture,
        "model" => parse_model,
        "collision" => parse_collision,
        "nav" => parse_nav,
        "text" => parse_text,
        "gamedata" => parse_gamedata,
        "mapdata" => parse_mapdata,
        "sco" => parse_sco,
        "shaderpack" => parse_shaderpack,
        "audio-config" => parse_audio_config,
        "save" => parse_save,
        "anim" => parse_anim,
        "audio-bank" => parse_audio_bank,
        "cutscene" => parse_cutscene,
        "effects" => parse_effects,
        "entity-meta" => parse_entity_meta,
        _ => return None,
    })
}

fn err(e: impl std::fmt::Display) -> String {
    e.to_string()
}

fn lower(name: &str) -> String {
    name.to_ascii_lowercase()
}

fn parse_archive(_: &str, b: &[u8]) -> Result<usize, String> {
    use lf_archive::Archive;
    Ok(lf_archive::open(&mut Cursor::new(b), None)
        .map_err(err)?
        .len())
}

fn parse_resource(_: &str, b: &[u8]) -> Result<usize, String> {
    Ok(lf_resource::Resource::parse(b).map_err(err)?.system().len())
}

fn parse_texture(_: &str, b: &[u8]) -> Result<usize, String> {
    Ok(lf_texture::Dictionary::parse(b).map_err(err)?.len())
}

fn parse_model(name: &str, b: &[u8]) -> Result<usize, String> {
    let res = lf_model::Resource::open(b).map_err(err)?;
    let name = lower(name);
    let geometries = if has_ext(&name, "wft") {
        lf_model::Fragment::parse(&res)
            .map_err(err)?
            .drawable
            .geometries()
            .count()
    } else if has_ext(&name, "wdd") {
        let d = lf_model::DrawableDictionary::parse(&res).map_err(err)?;
        d.entries.iter().map(|e| e.geometries().count()).sum()
    } else {
        lf_model::Drawable::parse(&res)
            .map_err(err)?
            .geometries()
            .count()
    };
    Ok(geometries)
}

fn parse_collision(_: &str, b: &[u8]) -> Result<usize, String> {
    Ok(lf_collision::parse(b).map_err(err)?.all_bounds().len())
}

fn parse_nav(name: &str, b: &[u8]) -> Result<usize, String> {
    let name = lower(name);
    if has_ext(&name, "wnv") {
        let r = lf_nav::rsc::Resource::parse(b).map_err(err)?;
        Ok(lf_nav::wnv::Tile::parse(r.payload())
            .map_err(err)?
            .poly_count() as usize)
    } else if has_ext(&name, "nod") {
        let n = lf_nav::nod::Nod::parse(b).map_err(err)?;
        let mut links = 0;
        for i in 0..n.node_count() {
            links += n.links_of(i).map_err(err)?.len();
        }
        Ok(links)
    } else {
        Ok(lf_nav::ipl::IplPaths::parse(&String::from_utf8_lossy(b))
            .nodes
            .len())
    }
}

fn parse_text(name: &str, b: &[u8]) -> Result<usize, String> {
    let name = lower(name);
    if has_ext(&name, "gxt") {
        Ok(lf_text::GxtFile::parse(b).map_err(err)?.entry_count())
    } else if name.starts_with("fonts") {
        Ok(lf_text::FontFile::parse(b).map_err(err)?.fonts.len())
    } else if name == "frontend_menus.xml" {
        Ok(lf_text::MenuFile::parse(b).map_err(err)?.sections.len())
    } else if name == "radiohud.dat" {
        lf_text::RadioHudFile::parse(b).map_err(err).map(|_| 1)
    } else if name == "hud.dat" {
        Ok(lf_text::HudFile::parse(b).map_err(err)?.sections.len())
    } else if name == "hudcolor.dat" {
        Ok(lf_text::HudColours::parse(b).map_err(err)?.sections.len())
    } else {
        Ok(lf_text::FrontendLayout::parse(b)
            .map_err(err)?
            .sections
            .len())
    }
}

fn parse_gamedata(name: &str, b: &[u8]) -> Result<usize, String> {
    let parsed = lf_gamedata::route::parse_file(name, b).map_err(err)?;
    Ok(parsed.counts().iter().map(|(_, n)| n).sum())
}

fn parse_mapdata(name: &str, b: &[u8]) -> Result<usize, String> {
    let name = lower(name);
    if has_ext(&name, "wpl") {
        Ok(lf_mapdata::wpl::WplFile::parse(b).map_err(err)?.len())
    } else if has_ext(&name, "ide") {
        Ok(lf_mapdata::ide::IdeFile::parse(b).map_err(err)?.len())
    } else if has_ext(&name, "ipl") {
        Ok(lf_mapdata::ipl::IplFile::parse(b).map_err(err)?.len())
    } else {
        Ok(lf_mapdata::loadlist::LoadList::parse(b)
            .map_err(err)?
            .directives
            .len())
    }
}

fn parse_sco(_: &str, b: &[u8]) -> Result<usize, String> {
    let script = lf_sco::container::load(b, None).map_err(|e| match e {
        lf_sco::LoadError::MissingKey => {
            "encrypted script: the benchmark needs a plain file".to_string()
        }
        other => other.to_string(),
    })?;
    Ok(lf_sco::decode_all(&script.code).map_err(err)?.len())
}

fn parse_shaderpack(_: &str, b: &[u8]) -> Result<usize, String> {
    Ok(lf_shaderpack::ShaderPack::parse(b)
        .map_err(err)?
        .programs()
        .count())
}

fn parse_audio_config(name: &str, b: &[u8]) -> Result<usize, String> {
    if lower(name).contains("speech") {
        Ok(lf_audio_config::speech::SpeechFile::parse(b)
            .map_err(err)?
            .contexts()
            .len())
    } else {
        Ok(lf_audio_config::container::MetaFile::parse(b)
            .map_err(err)?
            .objects()
            .len())
    }
}

fn parse_save(_: &str, b: &[u8]) -> Result<usize, String> {
    Ok(lf_save::SaveFile::parse(b).map_err(err)?.blocks().len())
}

fn parse_anim(_: &str, b: &[u8]) -> Result<usize, String> {
    Ok(lf_anim::AnimDictionary::parse_bytes(b).map_err(err)?.len())
}

fn parse_audio_bank(_: &str, b: &[u8]) -> Result<usize, String> {
    match lf_audio_bank::detect(b).map_err(err)? {
        lf_audio_bank::Container::Bank(bank) => Ok(bank.entries().map_err(err)?.len()),
        lf_audio_bank::Container::Streamed(s) => Ok(s.block_count() as usize),
    }
}

fn parse_cutscene(_: &str, b: &[u8]) -> Result<usize, String> {
    Ok(lf_cutscene::cut::CutsceneFile::parse(b)
        .map_err(err)?
        .groups
        .len())
}

fn parse_effects(_: &str, b: &[u8]) -> Result<usize, String> {
    if b.starts_with(b"RSC\x05") {
        return Ok(lf_effects::WpflFile::parse(b).map_err(err)?.entry_count());
    }
    let text = std::str::from_utf8(b).map_err(err)?;
    if text.trim_start().starts_with("<?xml") {
        Ok(lf_effects::emitter::parse(text).map_err(err)?.props.len())
    } else {
        Ok(lf_effects::FxFile::parse(text).map_err(err)?.row_count())
    }
}

fn parse_entity_meta(name: &str, b: &[u8]) -> Result<usize, String> {
    if has_ext(&lower(name), "wdr") {
        Ok(lf_entity_meta::parse_weapon(b).map_err(err)?.bone_count)
    } else {
        Ok(lf_entity_meta::parse_vehicle(b).map_err(err)?.bone_count)
    }
}
