//! Print what the translator reads and makes for every program in your own
//! `.fxc` shader containers: the reflection summary, the decoded listing
//! and the SPIR-V as text; or, with `--census`, one coverage census over
//! every distinct program in the files given.
//!
//! Run it on your own copy of the game. Its output is derived from game
//! files (a disassembly of the game's shaders): keep it on your machine;
//! never commit it, paste it into the repository, the devlog or an issue,
//! or share it.
//!
//! Usage: `lf_dxso_dump [OPTIONS] <FILE.fxc>...` (see `--help`).
//!
//! It reads only the files named and writes only to standard output, or to
//! the one file named by `--out`, which is never overwritten unless
//! `--force` is given and is never one of the inputs.

use lf_dxso_spirv::coverage::Census;
use lf_dxso_spirv::listing::listing;
use lf_dxso_spirv::spirv_text::{disassemble, reflection_summary};
use lf_dxso_spirv::validate::validate;
use lf_dxso_spirv::{Options, decode_bytes, translate_shader};
use lf_shaderpack::{ShaderPack, Stage};
use std::collections::HashSet;
use std::ffi::OsString;
use std::fs;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const HELP: &str = "\
lf_dxso_dump: show how the shader translator sees the programs in your own
.fxc shader containers.

USAGE:
    lf_dxso_dump [OPTIONS] <FILE.fxc>...

For every program of every file, prints a reflection summary (interface,
samplers, constants, validator verdict), the decoded listing (one
instruction per line, with the dword offsets translation errors report)
and the translated SPIR-V as text. Without --listing, --spirv or
--reflection all three are printed.

OPTIONS:
    --reflection        Print the reflection summary.
    --listing           Print the decoded listing.
    --spirv             Print the translated SPIR-V as text.
    --census            Print one coverage census over every distinct
                        program in the files instead: what fraction
                        translates, why the rest does not, and which
                        opcodes, registers, modifiers and declarations
                        the programs use.
    --program <vsN|psN> Only the Nth vertex or pixel program of each file.
    --out <FILE>        Write to FILE instead of standard output. An
                        existing FILE is not overwritten unless --force is
                        given; an input file is never overwritten.
    --force             Overwrite the --out file.
    -h, --help          Show this text.

YOUR FILES STAY YOURS:
    Everything this tool prints is derived from your own copy of the game
    (it is a disassembly of the game's shaders). Keep it on your machine:
    never commit it to the LibertyFlux repository (or any repository),
    paste it into the devlog or an issue, upload it or share it.
";

/// Which sections to print per program.
#[derive(Debug, Default, Clone, Copy)]
struct Sections {
    reflection: bool,
    listing: bool,
    spirv: bool,
}

/// Parsed command line.
#[derive(Debug, Default)]
struct Args {
    inputs: Vec<PathBuf>,
    sections: Sections,
    census: bool,
    program: Option<(Stage, usize)>,
    out: Option<PathBuf>,
    force: bool,
}

/// `vs0` or `ps12` to a stage and index.
fn parse_program(text: &str) -> Option<(Stage, usize)> {
    let stage = match text.get(..2)? {
        "vs" => Stage::Vertex,
        "ps" => Stage::Pixel,
        _ => return None,
    };
    Some((stage, text.get(2..)?.parse().ok()?))
}

/// `Ok(None)` asks for the help text.
fn parse_args(args: impl IntoIterator<Item = OsString>) -> Result<Option<Args>, String> {
    let mut a = Args::default();
    let mut it = args.into_iter();
    while let Some(arg) = it.next() {
        match arg.to_str() {
            Some("-h" | "--help") => return Ok(None),
            Some("--reflection") => a.sections.reflection = true,
            Some("--listing") => a.sections.listing = true,
            Some("--spirv") => a.sections.spirv = true,
            Some("--census") => a.census = true,
            Some("--force") => a.force = true,
            Some("--program") => {
                let value = it.next().and_then(|v| v.into_string().ok());
                a.program = Some(
                    value
                        .as_deref()
                        .and_then(parse_program)
                        .ok_or("--program needs a value such as vs0 or ps3")?,
                );
            }
            Some("--out") => {
                a.out = Some(PathBuf::from(it.next().ok_or("--out needs a file name")?));
            }
            Some(s) if s.starts_with('-') => return Err(format!("unknown option `{s}`")),
            _ => a.inputs.push(PathBuf::from(arg)),
        }
    }
    if a.inputs.is_empty() {
        return Err("no .fxc file given".to_string());
    }
    let chosen = a.sections.reflection || a.sections.listing || a.sections.spirv;
    if a.census && chosen {
        return Err(
            "--census prints only the census; leave out --reflection, --listing and --spirv"
                .to_string(),
        );
    }
    if !chosen {
        a.sections = Sections {
            reflection: true,
            listing: true,
            spirv: true,
        };
    }
    Ok(Some(a))
}

/// The output: standard output, or a new file that is not an input.
fn open_output(args: &Args) -> Result<Box<dyn Write>, String> {
    let Some(path) = &args.out else {
        return Ok(Box::new(BufWriter::new(io::stdout().lock())));
    };
    if let Ok(target) = fs::canonicalize(path) {
        for input in &args.inputs {
            if fs::canonicalize(input).is_ok_and(|i| i == target) {
                return Err(format!("{} is an input file", path.display()));
            }
        }
    }
    let mut options = fs::OpenOptions::new();
    options.write(true);
    if args.force {
        options.create(true).truncate(true);
    } else {
        options.create_new(true);
    }
    let file = options.open(path).map_err(|e| {
        if e.kind() == io::ErrorKind::AlreadyExists {
            format!(
                "{} already exists (use --force to overwrite)",
                path.display()
            )
        } else {
            format!("cannot create {}: {e}", path.display())
        }
    })?;
    Ok(Box::new(BufWriter::new(file)))
}

/// `vs0`, `ps3`, ... for every program of a container, in file order.
fn labelled<'p, 'a>(
    pack: &'p ShaderPack<'a>,
) -> impl Iterator<Item = (String, Stage, usize, &'p lf_shaderpack::Program<'a>)> {
    let mut counters = [0usize; 2];
    pack.programs().map(move |(stage, program)| {
        let (tag, n) = match stage {
            Stage::Vertex => ("vs", &mut counters[0]),
            Stage::Pixel => ("ps", &mut counters[1]),
        };
        let index = *n;
        *n += 1;
        (format!("{tag}{index}"), stage, index, program)
    })
}

fn wanted(args: &Args, stage: Stage, index: usize) -> bool {
    args.program.is_none_or(|p| p == (stage, index))
}

/// The per-program sections of one container.
fn dump_file(
    args: &Args,
    path: &Path,
    pack: &ShaderPack<'_>,
    out: &mut dyn Write,
) -> io::Result<()> {
    for (label, stage, index, program) in labelled(pack) {
        if !wanted(args, stage, index) {
            continue;
        }
        writeln!(
            out,
            "== {} {label}: {} bytes ==",
            path.display(),
            program.bytecode.len()
        )?;
        let shader = match decode_bytes(program.bytecode) {
            Ok(s) => s,
            Err(e) => {
                writeln!(out, "decode error: {e}\n")?;
                continue;
            }
        };
        let module = translate_shader(&shader, &Options::default());
        if args.sections.reflection {
            writeln!(out, "-- reflection --")?;
            match &module {
                Ok(m) => {
                    write!(out, "{}", reflection_summary(m))?;
                    match validate(&m.words) {
                        Ok(r) => writeln!(
                            out,
                            "validator: ok ({} instructions, {} functions, {} blocks)",
                            r.instructions, r.functions, r.blocks
                        )?,
                        Err(e) => writeln!(out, "validator: {e}")?,
                    }
                }
                Err(e) => writeln!(out, "translation: {e}")?,
            }
        }
        if args.sections.listing {
            writeln!(out, "-- listing --")?;
            write!(out, "{}", listing(&shader))?;
        }
        if args.sections.spirv {
            writeln!(out, "-- spirv --")?;
            match &module {
                Ok(m) => write!(out, "{}", disassemble(&m.words).text)?,
                Err(e) => writeln!(out, "not translated: {e}")?,
            }
        }
        writeln!(out)?;
    }
    Ok(())
}

fn main() -> ExitCode {
    let args = match parse_args(std::env::args_os().skip(1)) {
        Ok(Some(a)) => a,
        Ok(None) => {
            print!("{HELP}");
            return ExitCode::SUCCESS;
        }
        Err(e) => {
            eprintln!("lf_dxso_dump: {e} (see --help)");
            return ExitCode::from(2);
        }
    };
    let mut out = match open_output(&args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("lf_dxso_dump: {e}");
            return ExitCode::FAILURE;
        }
    };
    let mut failed = false;
    let mut census = Census::default();
    let mut distinct: HashSet<Vec<u8>> = HashSet::new();
    let mut programs = 0usize;
    for path in &args.inputs {
        let bytes = match fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("lf_dxso_dump: cannot read {}: {e}", path.display());
                failed = true;
                continue;
            }
        };
        let pack = match ShaderPack::parse(&bytes) {
            Ok(p) => p,
            Err(e) => {
                eprintln!(
                    "lf_dxso_dump: {}: container parse error: {e}",
                    path.display()
                );
                failed = true;
                continue;
            }
        };
        if args.census {
            for (_, stage, index, program) in labelled(&pack) {
                if !wanted(&args, stage, index) {
                    continue;
                }
                programs += 1;
                if distinct.insert(program.bytecode.to_vec()) {
                    census.add_bytes(program.bytecode);
                }
            }
        } else if let Err(e) = dump_file(&args, path, &pack, &mut *out) {
            eprintln!("lf_dxso_dump: cannot write the output: {e}");
            return ExitCode::FAILURE;
        }
    }
    if args.census {
        let written = writeln!(
            out,
            "{} files, {programs} programs, {} distinct; the census counts each distinct program once\n",
            args.inputs.len(),
            distinct.len()
        )
        .and_then(|()| write!(out, "{}", census.report()));
        if let Err(e) = written {
            eprintln!("lf_dxso_dump: cannot write the output: {e}");
            return ExitCode::FAILURE;
        }
    }
    if let Err(e) = out.flush() {
        eprintln!("lf_dxso_dump: cannot write the output: {e}");
        return ExitCode::FAILURE;
    }
    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
