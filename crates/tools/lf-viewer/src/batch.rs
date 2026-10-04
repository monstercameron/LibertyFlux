//! The `batch` command: convert every model and texture dictionary in a
//! folder, each into its own folder under the output folder.
//!
//! Inputs are the files in the folder (and, with `--recursive`, in its
//! subfolders) whose extension is `.wdr`, `.wdd`, `.wft` or `.wtd`, in
//! sorted order; every other file is counted as ignored, by extension.
//! Symbolic links to files are read; links to folders are not followed, so
//! the walk always ends, and the output folder is not walked when it lies
//! inside the input folder.
//!
//! Each input `<sub>/<name>.<ext>` is converted into `<out>/<sub>/<name>/`
//! exactly as the `model` or `texture` command would convert it into that
//! folder, under the same `--force` rule. Every path component is passed
//! through [`convert::sanitize`], and a folder name already given to
//! another input in the same place becomes `<name>_<ext>`, then
//! `<name>_<ext>_2`, and so on, so two inputs never share an output folder
//! and nothing is written outside `<out>`. `--textures` dictionaries are
//! read once and searched for every model.
//!
//! A file that cannot be converted (unreadable, not a valid file of its
//! kind, no exportable geometry or texture, outputs already present) is
//! skipped with its reason and the run goes on; so is a subfolder that
//! cannot be read. The run fails only when nothing at all was converted.
//! The log ends with the summary: what was converted, what was skipped and
//! why, and what was ignored.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::cli::{self, CliError};
use crate::convert::{self, Options};
use crate::info::FileKind;

/// What a folder walk found.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Found {
    /// Model and texture files, relative to the folder, sorted, with the
    /// kind their extension names.
    pub inputs: Vec<(PathBuf, FileKind)>,
    /// Other files, by lowercase extension (`.txt`, or `(none)`).
    pub ignored: BTreeMap<String, usize>,
    /// Subfolders that could not be read, relative to the folder, with the
    /// reason.
    pub unreadable: Vec<(PathBuf, String)>,
}

/// What a batch run did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Summary {
    /// Inputs converted, relative to the input folder, with their output
    /// folder relative to the output folder.
    pub converted: Vec<(PathBuf, PathBuf)>,
    /// Inputs and subfolders skipped, relative to the input folder, with
    /// the reason.
    pub skipped: Vec<(PathBuf, String)>,
    /// Files that are not model or texture files, by extension.
    pub ignored: BTreeMap<String, usize>,
}

/// Lists the model and texture files in `dir`, and in its subfolders when
/// `recursive`; `exclude` (the output folder) is never walked.
///
/// # Errors
///
/// The error reading `dir` itself. Unreadable subfolders are listed in
/// [`Found::unreadable`] instead.
pub fn find(dir: &Path, recursive: bool, exclude: Option<&Path>) -> io::Result<Found> {
    let exclude = exclude.and_then(|p| fs::canonicalize(p).ok());
    let mut found = Found::default();
    let entries = read_sorted(dir)?;
    walk(
        Path::new(""),
        entries,
        recursive,
        exclude.as_deref(),
        &mut found,
    );
    found.inputs.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(found)
}

fn read_sorted(dir: &Path) -> io::Result<Vec<fs::DirEntry>> {
    let mut entries = fs::read_dir(dir)?.collect::<io::Result<Vec<_>>>()?;
    entries.sort_by_key(fs::DirEntry::file_name);
    Ok(entries)
}

fn walk(
    rel: &Path,
    entries: Vec<fs::DirEntry>,
    recursive: bool,
    exclude: Option<&Path>,
    found: &mut Found,
) {
    for entry in entries {
        let rel_path = rel.join(entry.file_name());
        let file_type = match entry.file_type() {
            Ok(t) => t,
            Err(e) => {
                found.unreadable.push((rel_path, e.to_string()));
                continue;
            }
        };
        if file_type.is_dir() {
            let path = entry.path();
            let excluded = exclude.is_some_and(|x| fs::canonicalize(&path).is_ok_and(|p| p == x));
            if recursive && !excluded {
                match read_sorted(&path) {
                    Ok(sub) => walk(&rel_path, sub, recursive, exclude, found),
                    Err(e) => found.unreadable.push((rel_path, e.to_string())),
                }
            }
            continue;
        }
        let is_file = file_type.is_file()
            || (file_type.is_symlink() && fs::metadata(entry.path()).is_ok_and(|m| m.is_file()));
        if !is_file {
            continue;
        }
        match FileKind::from_extension(&rel_path) {
            Some(kind) => found.inputs.push((rel_path, kind)),
            None => *found.ignored.entry(extension_key(&rel_path)).or_default() += 1,
        }
    }
}

/// `.txt`, or `(none)` for a file without an extension.
fn extension_key(path: &Path) -> String {
    path.extension().map_or_else(
        || "(none)".to_string(),
        |e| format!(".{}", e.to_string_lossy().to_ascii_lowercase()),
    )
}

/// The output folder of every input, relative to the output folder: the
/// input's folder with every component sanitized, then its sanitized file
/// stem, made unique as the [module documentation](self) describes.
#[must_use]
pub fn output_folders(inputs: &[(PathBuf, FileKind)]) -> Vec<PathBuf> {
    let mut used: HashSet<String> = HashSet::new();
    inputs
        .iter()
        .map(|(rel, _)| {
            let parent: PathBuf = rel
                .parent()
                .map(|p| {
                    p.iter()
                        .map(|c| convert::sanitize(&c.to_string_lossy()))
                        .collect()
                })
                .unwrap_or_default();
            let stem = convert::sanitize(
                &rel.file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_default(),
            );
            let ext = convert::sanitize(
                &rel.extension()
                    .map(|e| e.to_string_lossy().to_ascii_lowercase())
                    .unwrap_or_default(),
            );
            let key = |name: &str| parent.join(name).to_string_lossy().to_ascii_lowercase();
            let mut name = stem.clone();
            let mut attempt = 1;
            while used.contains(&key(&name)) {
                attempt += 1;
                name = if attempt == 2 {
                    format!("{stem}_{ext}")
                } else {
                    format!("{stem}_{ext}_{}", attempt - 1)
                };
            }
            used.insert(key(&name));
            parent.join(name)
        })
        .collect()
}

/// Runs the batch command, logging each file and then the summary.
///
/// # Errors
///
/// [`CliError`] when the folder or a `--textures` dictionary cannot be
/// read, or when nothing could be converted (the summary is logged first).
pub(crate) fn run(
    input: &Path,
    texture_paths: &[PathBuf],
    out: &Path,
    force: bool,
    recursive: bool,
    options: Options,
    log: &mut dyn Write,
) -> Result<Summary, CliError> {
    let found = find(input, recursive, Some(out)).map_err(|error| CliError::Io {
        path: input.to_path_buf(),
        error,
    })?;
    let dictionaries = cli::read_dictionaries(texture_paths)?;
    let folders = output_folders(&found.inputs);
    let total = found.inputs.len();
    let mut summary = Summary {
        converted: Vec::new(),
        skipped: found.unreadable,
        ignored: found.ignored,
    };
    for (i, ((rel, kind), folder)) in found.inputs.iter().zip(&folders).enumerate() {
        let path = input.join(rel);
        let dest = out.join(folder);
        let _ = writeln!(
            log,
            "[{}/{total}] {} -> {}",
            i + 1,
            rel.display(),
            dest.display()
        );
        let result = match kind {
            FileKind::Textures => {
                cli::run_texture(&path, &dest, force, log).and_then(|(written, held)| {
                    if written > 0 {
                        Ok(())
                    } else if held == 0 {
                        Err(CliError::Empty("the dictionary holds no textures".into()))
                    } else {
                        Err(CliError::Empty(format!(
                            "none of its {held} textures could be decoded"
                        )))
                    }
                })
            }
            FileKind::Model(_) => {
                convert_model_file(&path, &dictionaries, &dest, force, options, log)
            }
        };
        match result {
            Ok(()) => summary.converted.push((rel.clone(), folder.clone())),
            Err(e) => {
                let _ = writeln!(log, "skipped: {e}");
                summary.skipped.push((rel.clone(), e.to_string()));
            }
        }
    }
    report(&summary, out, log);
    if summary.converted.is_empty() {
        return Err(CliError::Empty(if total == 0 {
            format!(
                "{}: no .wdr, .wdd, .wft or .wtd files found",
                input.display()
            )
        } else {
            format!("none of the {total} model and texture files could be converted")
        }));
    }
    Ok(summary)
}

/// One model file, as the `model` command converts it (kind from the
/// extension).
fn convert_model_file(
    path: &Path,
    dictionaries: &[(String, lf_texture::Dictionary)],
    dest: &Path,
    force: bool,
    options: Options,
    log: &mut dyn Write,
) -> Result<(), CliError> {
    let bytes = cli::read(path)?;
    let res = lf_model::Resource::open(&bytes).map_err(|e| cli::format_err(path, &e))?;
    cli::convert_model(path, &res, dictionaries, None, dest, force, options, log)
}

/// The closing summary.
fn report(summary: &Summary, out: &Path, log: &mut dyn Write) {
    let ignored: usize = summary.ignored.values().sum();
    let _ = writeln!(
        log,
        "batch: {} converted, {} skipped, {ignored} ignored; output in {}",
        summary.converted.len(),
        summary.skipped.len(),
        out.display()
    );
    if !summary.skipped.is_empty() {
        let _ = writeln!(log, "skipped:");
        for (path, reason) in &summary.skipped {
            let _ = writeln!(log, "  {}: {reason}", path.display());
        }
    }
    if ignored > 0 {
        let by_extension: Vec<String> = summary
            .ignored
            .iter()
            .map(|(ext, n)| format!("{ext} {n}"))
            .collect();
        let _ = writeln!(
            log,
            "ignored (not .wdr, .wdd, .wft or .wtd): {}",
            by_extension.join(", ")
        );
    }
}
