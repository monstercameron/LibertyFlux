//! Archive-level helpers: read `.cut` files out of `cuts.img` and resolve
//! the references they contain.
//!
//! These helpers build on [`lf_archive`] (table decryption, entry reads) and
//! [`lf_resource`] (header checks on `.wad` animation payloads). They never
//! write to the game folder and never expose the archive key.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::BuildHasher;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use lf_archive::img::ImgArchive;
use lf_archive::rpf::RpfArchive;
use lf_archive::{Archive, crypto::Key};

use crate::cut::CutsceneFile;
use crate::{Error, Result};

/// One parsed `.cut` entry and where it came from.
#[derive(Clone, Debug)]
pub struct ParsedCut {
    /// Archive file name (for example `cuts.img`).
    pub archive: String,
    /// Episode derived from the archive path (`base`, `tlad` or `tbogt`).
    pub episode: String,
    /// Entry name inside the archive.
    pub name: String,
    /// Stored payload size in bytes.
    pub size: u64,
    /// The parsed file.
    pub file: CutsceneFile,
}

/// Recursively find archives called `file_name` under `game_dir` (sorted).
#[must_use]
pub fn find_archives(game_dir: &Path, file_name: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![game_dir.to_path_buf()];
    while let Some(top) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&top) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path
                .file_name()
                .is_some_and(|n| n.eq_ignore_ascii_case(file_name))
            {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

/// Episode tag derived from an archive path.
#[must_use]
pub fn episode_of(path: &Path) -> String {
    let s = path.to_string_lossy();
    if s.contains("TBoGT") {
        "tbogt".to_string()
    } else if s.contains("TLAD") {
        "tlad".to_string()
    } else {
        "base".to_string()
    }
}

/// Open an IMG archive's table of contents.
///
/// # Errors
///
/// Returns an error when the file cannot be read or its table of
/// contents fails to parse.
pub fn open_img(path: &Path, key: &Key) -> Result<(ImgArchive, BufReader<File>)> {
    let mut reader = BufReader::new(File::open(path)?);
    let archive = ImgArchive::open(&mut reader, Some(key))?;
    Ok((archive, reader))
}

/// Parse every `.cut` entry in one `cuts.img` archive.
///
/// # Errors
///
/// Returns an error when the archive cannot be opened or a `.cut`
/// entry fails to parse.
pub fn parse_cuts_in_archive(path: &Path, key: &Key) -> Result<Vec<ParsedCut>> {
    let (archive, mut reader) = open_img(path, key)?;
    let archive_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let episode = episode_of(path);
    let mut out = Vec::new();
    for (index, entry) in archive.entries().iter().enumerate() {
        let name = entry.name.clone().unwrap_or_else(|| entry.path.clone());
        if !name.to_ascii_lowercase().ends_with(".cut") {
            continue;
        }
        let bytes = archive.read_file(&mut reader, index, Some(key))?;
        let file = CutsceneFile::parse(&bytes)?;
        out.push(ParsedCut {
            archive: archive_name.clone(),
            episode: episode.clone(),
            name,
            size: entry.size,
            file,
        });
    }
    Ok(out)
}

/// Lowercased entry names of an archive (for reference resolution).
///
/// # Errors
///
/// Returns an error when the archive cannot be opened.
pub fn entry_names_lower(path: &Path, key: &Key) -> Result<HashSet<String>> {
    let (archive, _) = open_img(path, key)?;
    Ok(archive
        .entries()
        .iter()
        .filter_map(|e| e.name.clone())
        .map(|n| n.to_ascii_lowercase())
        .collect())
}

/// Entry-name stems (lowercased, extension stripped) of an archive.
///
/// # Errors
///
/// Returns an error when the archive cannot be opened.
pub fn entry_stems_lower(path: &Path, key: &Key) -> Result<HashSet<String>> {
    let (archive, _) = open_img(path, key)?;
    Ok(archive
        .entries()
        .iter()
        .filter_map(|e| e.name.clone())
        .map(|n| {
            let lower = n.to_ascii_lowercase();
            match lower.rfind('.') {
                Some(i) => lower[..i].to_string(),
                None => lower,
            }
        })
        .collect())
}

/// Read the first `n` bytes of one entry without loading the whole payload.
///
/// `.wad` animation files are megabytes; header checks only need the prefix.
/// The reader position is restored before returning.
///
/// # Errors
///
/// Returns an error when the index is out of range or the bytes cannot
/// be read.
pub fn read_entry_prefix<R: Read + Seek>(
    archive: &ImgArchive,
    reader: &mut R,
    index: usize,
    n: usize,
) -> Result<Vec<u8>> {
    let entry = archive.entries().get(index).ok_or_else(|| {
        Error::Archive(lf_archive::Error::EntryOutOfRange {
            path: format!("entry #{index}"),
        })
    })?;
    let pos = reader.stream_position()?;
    reader.seek(SeekFrom::Start(entry.offset))?;
    let stored = usize::try_from(entry.size).unwrap_or(usize::MAX);
    let mut buf = vec![0u8; n.min(stored)];
    reader.read_exact(&mut buf)?;
    reader.seek(SeekFrom::Start(pos))?;
    Ok(buf)
}

/// Check the `RSC5` resource headers of every `.wad` entry in an archive.
///
/// Returns `(name, type_id, codec_name)` per entry. Only the 16-byte prefix
/// of each payload is read; nothing is inflated.
///
/// # Errors
///
/// Returns an error when the archive cannot be opened or a `.wad`
/// prefix fails to parse as a resource header.
pub fn wad_headers(path: &Path, key: &Key) -> Result<Vec<(String, u32, String)>> {
    let (archive, mut reader) = open_img(path, key)?;
    let mut out = Vec::new();
    for (index, entry) in archive.entries().iter().enumerate() {
        let name = entry.name.clone().unwrap_or_else(|| entry.path.clone());
        if !name.to_ascii_lowercase().ends_with(".wad") {
            continue;
        }
        let prefix = read_entry_prefix(&archive, &mut reader, index, 16)?;
        let header = lf_resource::Header::parse(&prefix)?;
        out.push((name, header.kind.raw(), format!("{:?}", header.codec)));
    }
    Ok(out)
}

/// File-entry hashes of an RPF archive (RPF3 names are hashes, not strings).
///
/// # Errors
///
/// Returns an error when the archive cannot be opened.
pub fn rpf_file_hashes(path: &Path, key: &Key) -> Result<HashSet<u32>> {
    let mut reader = BufReader::new(File::open(path)?);
    let archive = RpfArchive::open(&mut reader, Some(key))?;
    Ok(archive
        .entries()
        .iter()
        .filter(|e| e.is_file())
        .filter_map(|e| e.hash)
        .collect())
}

/// Candidate spellings of an `AUDIO` name, hashed for `cutscenes.rpf` lookup.
///
/// RPF3 entries are keyed by Jenkins hash, so the name's normalisation is
/// unknown; these candidates cover the plausible spellings. Returns
/// `(candidate, hash)` pairs.
#[must_use]
pub fn audio_hash_candidates(name: &str) -> Vec<(String, u32)> {
    let mut out = Vec::new();
    for candidate in [
        name.to_string(),
        format!("cutscenes/{name}"),
        format!("audio/{name}"),
        format!("cutscenes/{name}.ivaud"),
    ] {
        out.push((candidate.clone(), lf_archive::hash::name_hash(&candidate)));
    }
    out
}

/// One row of the per-cutscene catalog: durations, object counts, references.
#[derive(Clone, Debug)]
pub struct CatalogRow {
    /// `episode/name.cut`.
    pub file: String,
    /// Cutscene index within the file.
    pub group: usize,
    /// Number of animation sections.
    pub sections: usize,
    /// Sum of the section durations in milliseconds.
    pub duration_ms: f32,
    /// Number of `MODELS` rows across all sections.
    pub models: usize,
    /// Number of `TEXT` subtitle rows.
    pub subtitles: usize,
    /// Distinct `ANIM` stems referenced.
    pub anims: Vec<String>,
    /// Distinct `AUDIO` names referenced.
    pub audios: Vec<String>,
}

/// Build catalog rows for parsed cuts.
#[must_use]
pub fn catalog_rows(cuts: &[ParsedCut]) -> Vec<CatalogRow> {
    let mut rows = Vec::new();
    for cut in cuts {
        for (group, g) in cut.file.groups.iter().enumerate() {
            let mut anims: Vec<String> = g
                .sections
                .iter()
                .flat_map(|s| s.anims.iter().cloned())
                .collect();
            anims.sort();
            anims.dedup();
            let mut audios: Vec<String> = g
                .sections
                .iter()
                .flat_map(|s| s.audios.iter().cloned())
                .collect();
            audios.sort();
            audios.dedup();
            rows.push(CatalogRow {
                file: format!("{}/{}", cut.episode, cut.name),
                group,
                sections: g.sections.len(),
                duration_ms: g.duration_ms(),
                models: g.model_count(),
                subtitles: g.texts.len(),
                anims,
                audios,
            });
        }
    }
    rows
}

/// Render catalog rows as JSON (hand-rolled; names need no escaping).
#[must_use]
pub fn catalog_json(rows: &[CatalogRow]) -> String {
    use std::fmt::Write as _;
    fn esc(s: &str) -> String {
        s.replace('\\', "\\\\").replace('"', "\\\"")
    }
    let mut out = String::from("[\n");
    for (i, r) in rows.iter().enumerate() {
        let anims = r
            .anims
            .iter()
            .map(|a| format!("\"{}\"", esc(a)))
            .collect::<Vec<_>>()
            .join(", ");
        let audios = r
            .audios
            .iter()
            .map(|a| format!("\"{}\"", esc(a)))
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            out,
            "  {{\"file\": \"{}\", \"group\": {}, \"sections\": {}, \
             \"duration_ms\": {:.3}, \"models\": {}, \"subtitles\": {}, \
             \"anims\": [{}], \"audios\": [{}]}}",
            esc(&r.file),
            r.group,
            r.sections,
            r.duration_ms,
            r.models,
            r.subtitles,
            anims,
            audios
        )
        .expect("write to String cannot fail");
        if i + 1 < rows.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("]\n");
    out
}

/// Count unresolved `ANIM` references (case-insensitive `stem + .wad`).
///
/// Returns `(resolved, missing)` where missing holds `(file, anim)` pairs.
#[must_use]
pub fn check_anims<MapHasher: BuildHasher, SetHasher: BuildHasher>(
    cuts: &[ParsedCut],
    wad_names_by_episode: &HashMap<String, HashSet<String, SetHasher>, MapHasher>,
) -> (usize, Vec<(String, String)>) {
    let mut resolved = 0usize;
    let mut missing = Vec::new();
    for cut in cuts {
        let wads = wad_names_by_episode.get(&cut.episode);
        for g in &cut.file.groups {
            for s in &g.sections {
                for anim in &s.anims {
                    let want = format!("{}.wad", anim.to_ascii_lowercase());
                    let found = wads.is_some_and(|w| w.contains(&want));
                    if found {
                        resolved += 1;
                    } else {
                        missing.push((format!("{}/{}", cut.episode, cut.name), anim.clone()));
                    }
                }
            }
        }
    }
    (resolved, missing)
}
