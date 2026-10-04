//! The command line: argument parsing, reading inputs, writing outputs.
//!
//! The tool reads only the files named on the command line (for `batch`,
//! the files in the folder named) and writes only inside the `--out`
//! folder; `info` writes nothing. It never overwrites a file unless
//! `--force` is given, and checks every planned output of an input before
//! writing the first one.

use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use lf_model::rsc5::{TYPE_DRAWABLE, TYPE_FRAGMENT};
use lf_model::{Drawable, DrawableDictionary, Fragment};
use lf_texture::Dictionary;

use crate::batch;
use crate::convert::{self, LodChoice, Options, TEXTURE_DIR};
use crate::gltf;
use crate::info::{self, FileKind};

/// The `--help` text.
pub const HELP: &str = "\
lf-viewer: convert a model and its textures from your own copy of the game
into standard files a viewer opens (glTF 2.0 and PNG).

USAGE:
    lf-viewer model <MODEL> --out <DIR> [OPTIONS]
    lf-viewer texture <TEXTURES.wtd> --out <DIR> [--force]
    lf-viewer info <FILE> [--json] [--kind <wdr|wdd|wft|wtd>]
    lf-viewer batch <FOLDER> --out <DIR> [--recursive] [OPTIONS]
    lf-viewer --help

MODEL is a drawable (.wdr), drawable dictionary (.wdd) or fragment (.wft).
The model command writes <DIR>/<name>.gltf, <DIR>/<name>.bin and the
textures it uses as <DIR>/textures/*.png. The texture command writes every
texture of a dictionary as <DIR>/*.png.

The info command prints what a model or texture dictionary holds (meshes
and their vertex, index and triangle counts, vertex layouts, materials and
the textures they name, texture formats, sizes and mip levels) and writes
nothing.

The batch command converts every .wdr, .wdd, .wft and .wtd file in FOLDER,
each into its own folder <DIR>/<name>/ as the model and texture commands
would, and ends with a summary of what was converted and what was skipped
and why. It writes nothing outside <DIR>, and fails only when nothing
could be converted.

MODEL OPTIONS:
    --textures <FILE.wtd>   A texture dictionary to take textures from;
                            repeat for several. Searched after any textures
                            embedded in the model.
    --lod <N>               Export level of detail N (default 0, the most
                            detailed).
    --all-lods              Export every level of detail as its own mesh.
    --kind <wdr|wdd|wft>    Read the model as this kind instead of guessing
                            from the file extension.
    --flip-winding          Reverse triangle winding (for viewers that cull
                            back faces and show the model inside out).
    --vertex-colors         Export vertex colours (off by default: the game
                            appears to use them for baked lighting).
    --keep-z-up             Do not rotate the game's +Z-up axes to glTF's +Y.

INFO OPTIONS:
    --json                  Print JSON instead of text.
    --kind <wdr|wdd|wft|wtd>
                            Read the file as this kind instead of guessing
                            from the extension and the file header.

BATCH OPTIONS:
    --recursive             Also convert the files in subfolders, mirroring
                            them under <DIR>.
    The model options --textures, --lod, --all-lods, --flip-winding,
    --vertex-colors and --keep-z-up apply to every model.

COMMON OPTIONS:
    --out <DIR>             Output folder (created if missing). Required by
                            every command but info, which takes none.
    --force                 Overwrite existing files.
    -h, --help              Show this text.

YOUR FILES STAY YOURS:
    Everything this tool writes is derived from your own copy of the game.
    Keep it on your machine: never commit it to the LibertyFlux repository
    (or any repository), upload it or share it. The project needs a copy of
    the game you bought (on Steam, from an authorised key seller or from a
    retailer); this tool does not help anyone without one.
";

/// Why a run failed.
#[derive(Debug)]
pub enum CliError {
    /// The arguments are wrong; the message says how.
    Usage(String),
    /// A file could not be read or written.
    Io {
        /// The file involved.
        path: PathBuf,
        /// The error.
        error: io::Error,
    },
    /// An input file is not a valid file of the expected format.
    Format {
        /// The file involved.
        path: PathBuf,
        /// What the reader reported.
        message: String,
    },
    /// An output file exists and `--force` was not given.
    Exists(PathBuf),
    /// The model held nothing exportable.
    Empty(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Usage(msg) => write!(f, "{msg} (see --help)"),
            CliError::Io { path, error } => write!(f, "{}: {error}", path.display()),
            CliError::Format { path, message } => write!(f, "{}: {message}", path.display()),
            CliError::Exists(path) => {
                write!(
                    f,
                    "{} already exists (use --force to overwrite)",
                    path.display()
                )
            }
            CliError::Empty(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for CliError {}

/// The kind of model file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelKind {
    /// A single drawable (`.wdr`).
    Drawable,
    /// A drawable dictionary (`.wdd`).
    Dictionary,
    /// A fragment (`.wft`).
    Fragment,
}

impl ModelKind {
    fn parse(s: &str) -> Option<ModelKind> {
        match s.to_ascii_lowercase().as_str() {
            "wdr" => Some(ModelKind::Drawable),
            "wdd" => Some(ModelKind::Dictionary),
            "wft" => Some(ModelKind::Fragment),
            _ => None,
        }
    }
}

/// A parsed command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Print the help text.
    Help,
    /// Convert a model.
    Model {
        /// The model file.
        input: PathBuf,
        /// Texture dictionaries to search.
        textures: Vec<PathBuf>,
        /// Kind override.
        kind: Option<ModelKind>,
        /// Output folder.
        out: PathBuf,
        /// Overwrite existing files.
        force: bool,
        /// Conversion settings.
        options: Options,
    },
    /// Convert every texture of a dictionary.
    Texture {
        /// The dictionary file.
        input: PathBuf,
        /// Output folder.
        out: PathBuf,
        /// Overwrite existing files.
        force: bool,
    },
    /// Print what a model or texture dictionary holds; writes nothing.
    Info {
        /// The file.
        input: PathBuf,
        /// Kind override.
        kind: Option<FileKind>,
        /// Print JSON instead of text.
        json: bool,
    },
    /// Convert every model and texture dictionary in a folder.
    Batch {
        /// The folder.
        input: PathBuf,
        /// Texture dictionaries to search for every model.
        textures: Vec<PathBuf>,
        /// Output folder.
        out: PathBuf,
        /// Overwrite existing files.
        force: bool,
        /// Descend into subfolders.
        recursive: bool,
        /// Conversion settings for every model.
        options: Options,
    },
}

/// Parses arguments (without the program name).
///
/// # Errors
///
/// Returns [`CliError::Usage`] for unknown options, missing values or a
/// missing input or output.
pub fn parse_args(args: &[OsString]) -> Result<Command, CliError> {
    let usage = |m: &str| CliError::Usage(m.to_string());
    let mut it = args.iter();
    let Some(first) = it.next() else {
        return Ok(Command::Help);
    };
    let sub = first.to_string_lossy();
    if matches!(sub.as_ref(), "-h" | "--help" | "help") {
        return Ok(Command::Help);
    }
    if sub == "info" {
        return parse_info(it);
    }
    if sub != "model" && sub != "texture" && sub != "batch" {
        return Err(usage(&format!("unknown command `{sub}`")));
    }
    // The batch command takes the model options (but not --kind).
    let models = sub == "model" || sub == "batch";
    let mut input = None;
    let mut out = None;
    let mut textures = Vec::new();
    let mut kind = None;
    let mut force = false;
    let mut recursive = false;
    let mut options = Options::default();
    while let Some(arg) = it.next() {
        let text = arg.to_string_lossy();
        let mut value = |name: &str| {
            it.next()
                .cloned()
                .ok_or_else(|| usage(&format!("{name} needs a value")))
        };
        match text.as_ref() {
            "-h" | "--help" => return Ok(Command::Help),
            "--out" => out = Some(PathBuf::from(value("--out")?)),
            "--force" => force = true,
            "--textures" if models => textures.push(PathBuf::from(value("--textures")?)),
            "--lod" if models => {
                let v = value("--lod")?;
                let n = v
                    .to_string_lossy()
                    .parse::<usize>()
                    .map_err(|_| usage("--lod needs a whole number"))?;
                options.lod = LodChoice::Index(n);
            }
            "--all-lods" if models => options.lod = LodChoice::All,
            "--kind" if sub == "model" => {
                let v = value("--kind")?;
                kind = Some(
                    ModelKind::parse(&v.to_string_lossy())
                        .ok_or_else(|| usage("--kind must be wdr, wdd or wft"))?,
                );
            }
            "--flip-winding" if models => options.flip_winding = true,
            "--vertex-colors" if models => options.vertex_colors = true,
            "--keep-z-up" if models => options.z_up_to_y_up = false,
            "--recursive" if sub == "batch" => recursive = true,
            t if t.starts_with('-') => {
                return Err(usage(&format!("unknown option `{t}` for `{sub}`")));
            }
            _ => {
                if input.is_some() {
                    return Err(usage(&format!("unexpected extra argument `{text}`")));
                }
                input = Some(PathBuf::from(arg));
            }
        }
    }
    let input = input.ok_or_else(|| {
        usage(if sub == "batch" {
            "no input folder given"
        } else {
            "no input file given"
        })
    })?;
    let out = out.ok_or_else(|| usage("--out is required"))?;
    Ok(match sub.as_ref() {
        "model" => Command::Model {
            input,
            textures,
            kind,
            out,
            force,
            options,
        },
        "batch" => Command::Batch {
            input,
            textures,
            out,
            force,
            recursive,
            options,
        },
        _ => Command::Texture { input, out, force },
    })
}

/// Parses the arguments after `info`.
fn parse_info<'a>(mut it: impl Iterator<Item = &'a OsString>) -> Result<Command, CliError> {
    let usage = |m: &str| CliError::Usage(m.to_string());
    let mut input = None;
    let mut kind = None;
    let mut json = false;
    while let Some(arg) = it.next() {
        let text = arg.to_string_lossy();
        match text.as_ref() {
            "-h" | "--help" => return Ok(Command::Help),
            "--json" => json = true,
            "--kind" => {
                let v = it.next().ok_or_else(|| usage("--kind needs a value"))?;
                kind = Some(
                    FileKind::parse(&v.to_string_lossy())
                        .ok_or_else(|| usage("--kind must be wdr, wdd, wft or wtd"))?,
                );
            }
            "--out" | "--force" => {
                return Err(usage(&format!(
                    "`info` writes nothing, so it takes no `{text}`"
                )));
            }
            t if t.starts_with('-') => {
                return Err(usage(&format!("unknown option `{t}` for `info`")));
            }
            _ => {
                if input.is_some() {
                    return Err(usage(&format!("unexpected extra argument `{text}`")));
                }
                input = Some(PathBuf::from(arg));
            }
        }
    }
    let input = input.ok_or_else(|| usage("no input file given"))?;
    Ok(Command::Info { input, kind, json })
}

pub(crate) fn read(path: &Path) -> Result<Vec<u8>, CliError> {
    fs::read(path).map_err(|error| CliError::Io {
        path: path.to_path_buf(),
        error,
    })
}

pub(crate) fn format_err(path: &Path, e: &dyn fmt::Display) -> CliError {
    CliError::Format {
        path: path.to_path_buf(),
        message: e.to_string(),
    }
}

/// A file name stem for the outputs, from the input's name.
fn stem_of(path: &Path) -> String {
    convert::sanitize(
        &path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default(),
    )
}

/// Writes every planned file, refusing to overwrite unless `force`.
fn write_all(files: &[(PathBuf, Vec<u8>)], force: bool) -> Result<(), CliError> {
    if !force && let Some((path, _)) = files.iter().find(|(p, _)| p.exists()) {
        return Err(CliError::Exists(path.clone()));
    }
    for (path, bytes) in files {
        let io_err = |error| CliError::Io {
            path: path.clone(),
            error,
        };
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(io_err)?;
        }
        if force {
            fs::write(path, bytes).map_err(io_err)?;
        } else {
            // create_new closes the gap between the check and the write.
            let mut f = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)
                .map_err(io_err)?;
            f.write_all(bytes).map_err(io_err)?;
        }
    }
    Ok(())
}

/// Picks the model kind: the override, else the extension, else the
/// resource type (drawable first, then dictionary).
pub(crate) fn guess_kind(path: &Path, res: &lf_model::Resource) -> ModelKind {
    if let Some(k) = path
        .extension()
        .and_then(|e| ModelKind::parse(&e.to_string_lossy()))
    {
        return k;
    }
    match res.kind {
        TYPE_FRAGMENT => ModelKind::Fragment,
        TYPE_DRAWABLE if Drawable::parse(res).is_err() => ModelKind::Dictionary,
        _ => ModelKind::Drawable,
    }
}

fn run_model(
    input: &Path,
    texture_paths: &[PathBuf],
    kind: Option<ModelKind>,
    out_dir: &Path,
    force: bool,
    options: Options,
    log: &mut dyn Write,
) -> Result<(), CliError> {
    let bytes = read(input)?;
    let res = lf_model::Resource::open(&bytes).map_err(|e| format_err(input, &e))?;
    let dictionaries = read_dictionaries(texture_paths)?;
    convert_model(
        input,
        &res,
        &dictionaries,
        kind,
        out_dir,
        force,
        options,
        log,
    )
}

/// Reads the `--textures` dictionaries, tagged with their paths.
pub(crate) fn read_dictionaries(paths: &[PathBuf]) -> Result<Vec<(String, Dictionary)>, CliError> {
    let mut dictionaries = Vec::new();
    for path in paths {
        let dict = Dictionary::parse(&read(path)?).map_err(|e| format_err(path, &e))?;
        dictionaries.push((path.display().to_string(), dict));
    }
    Ok(dictionaries)
}

/// Converts an opened model resource and writes its glTF, buffer and
/// textures into `out_dir` (the `model` command after reading its inputs).
#[allow(clippy::too_many_arguments)]
pub(crate) fn convert_model(
    input: &Path,
    res: &lf_model::Resource,
    dictionaries: &[(String, Dictionary)],
    kind: Option<ModelKind>,
    out_dir: &Path,
    force: bool,
    options: Options,
    log: &mut dyn Write,
) -> Result<(), CliError> {
    let stem = stem_of(input);
    let kind = kind.unwrap_or_else(|| guess_kind(input, res));
    let converted = match kind {
        ModelKind::Drawable => {
            let d = Drawable::parse(res).map_err(|e| format_err(input, &e))?;
            convert::convert(&stem, &[(stem.clone(), &d)], res, dictionaries, options)
        }
        ModelKind::Fragment => {
            let f = Fragment::parse(res).map_err(|e| format_err(input, &e))?;
            convert::convert(
                &stem,
                &[(stem.clone(), &f.drawable)],
                res,
                dictionaries,
                options,
            )
        }
        ModelKind::Dictionary => {
            let dict = DrawableDictionary::parse(res).map_err(|e| format_err(input, &e))?;
            let labelled: Vec<(String, &Drawable)> = dict
                .entries
                .iter()
                .enumerate()
                .map(|(i, d)| {
                    let hash = dict.hashes.get(i).copied().unwrap_or(0);
                    (format!("{stem}_{hash:08x}"), d)
                })
                .collect();
            convert::convert(&stem, &labelled, res, dictionaries, options)
        }
    };
    if converted.scene.meshes.is_empty() {
        for w in &converted.warnings {
            let _ = writeln!(log, "warning: {w}");
        }
        return Err(CliError::Empty(format!(
            "{}: no geometry could be exported",
            input.display()
        )));
    }
    let bin_name = format!("{stem}.bin");
    let (json, bin) = gltf::build(&converted.scene, &bin_name);
    let mut files = vec![
        (out_dir.join(format!("{stem}.gltf")), json.into_bytes()),
        (out_dir.join(&bin_name), bin),
    ];
    for t in &converted.textures {
        files.push((out_dir.join(TEXTURE_DIR).join(&t.file_name), t.png.clone()));
    }
    write_all(&files, force)?;
    for w in &converted.warnings {
        let _ = writeln!(log, "warning: {w}");
    }
    let _ = writeln!(
        log,
        "wrote {} ({} meshes, {} vertices, {} triangles, {} textures)",
        files[0].0.display(),
        converted.scene.meshes.len(),
        converted.vertices,
        converted.triangles,
        converted.textures.len()
    );
    Ok(())
}

/// The `texture` command. Returns the textures written and the textures
/// the dictionary holds.
pub(crate) fn run_texture(
    input: &Path,
    out_dir: &Path,
    force: bool,
    log: &mut dyn Write,
) -> Result<(usize, usize), CliError> {
    let dict = Dictionary::parse(&read(input)?).map_err(|e| format_err(input, &e))?;
    let (pngs, warnings) = convert::dictionary_to_pngs(&dict);
    let files: Vec<(PathBuf, Vec<u8>)> = pngs
        .into_iter()
        .map(|t| (out_dir.join(t.file_name), t.png))
        .collect();
    write_all(&files, force)?;
    for w in &warnings {
        let _ = writeln!(log, "warning: {w}");
    }
    let _ = writeln!(
        log,
        "wrote {} of {} textures to {}",
        files.len(),
        dict.len(),
        out_dir.display()
    );
    Ok((files.len(), dict.len()))
}

/// The `info` command: describes the file on `log`, writes no file.
fn run_info(
    input: &Path,
    kind: Option<FileKind>,
    json: bool,
    log: &mut dyn Write,
) -> Result<(), CliError> {
    let bytes = read(input)?;
    let described = info::inspect(input, &bytes, kind).map_err(|message| CliError::Format {
        path: input.to_path_buf(),
        message,
    })?;
    let name = input.display().to_string();
    let text = if json {
        let mut j = described.to_json(&name).to_json_string();
        j.push('\n');
        j
    } else {
        described.to_text(&name)
    };
    log.write_all(text.as_bytes())
        .map_err(|error| CliError::Io {
            path: PathBuf::from("<output>"),
            error,
        })
}

/// Runs the tool with `args` (without the program name), writing progress
/// and warnings to `log`.
///
/// # Errors
///
/// Returns [`CliError`] for bad arguments, unreadable or invalid inputs,
/// existing outputs without `--force`, or write failures.
pub fn run(args: &[OsString], log: &mut dyn Write) -> Result<(), CliError> {
    match parse_args(args)? {
        Command::Help => {
            let _ = log.write_all(HELP.as_bytes());
            Ok(())
        }
        Command::Model {
            input,
            textures,
            kind,
            out,
            force,
            options,
        } => run_model(&input, &textures, kind, &out, force, options, log),
        Command::Texture { input, out, force } => run_texture(&input, &out, force, log).map(|_| ()),
        Command::Info { input, kind, json } => run_info(&input, kind, json, log),
        Command::Batch {
            input,
            textures,
            out,
            force,
            recursive,
            options,
        } => batch::run(&input, &textures, &out, force, recursive, options, log).map(|_| ()),
    }
}
