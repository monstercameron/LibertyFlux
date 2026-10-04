//! `lf-inspect`: one command-line tool for every file format the project
//! reads. It replaces the per-crate example programs that used to live
//! under `crates/formats/*/examples` (their summaries print exactly as
//! before) and adds listings, JSON-ish dumps and parse benchmarks.
//!
//! ```text
//! lf-inspect <format> [<command>] <args...>   command defaults to summarize
//! lf-inspect formats                          formats and their commands
//! lf-inspect bench <format> <file> [--iters N]
//! lf-inspect bench synthetic [--iters N]
//! ```
//!
//! Every command only reads. Files come from a copy of the game the person
//! running the tool owns; where an archive key is needed it is located in
//! that copy's executable at run time and kept in memory only. Nothing from
//! the game is compiled into the tool: the synthetic benchmark generates
//! its own inputs.
//!
//! Exit codes: 0 on success, 1 when input cannot be read or parsed, 2 for
//! a usage mistake.

pub mod bench;
pub mod cli;
pub mod formats;
pub mod synth;

pub use cli::{CliError, CliResult, Io};

/// Top-level usage text.
pub const USAGE: &str = "\
usage: lf-inspect <format> [<command>] <args...>
       lf-inspect formats
       lf-inspect bench <format> <file> [--iters N]
       lf-inspect bench synthetic [--iters N]
The command defaults to summarize. Run `lf-inspect formats` for the list.";

/// Run the tool on `args` (without the program name).
///
/// # Errors
///
/// Returns the failing command's [`CliError`]; the caller prints its
/// message to standard error and exits with its code.
pub fn run(args: &[String], io: &mut Io) -> CliResult {
    let Some((first, rest)) = args.split_first() else {
        return Err(CliError::usage(USAGE));
    };
    match first.as_str() {
        "help" | "--help" | "-h" => {
            writeln!(io.out, "{USAGE}")?;
            Ok(())
        }
        "formats" => {
            for f in formats::FORMATS {
                writeln!(
                    io.out,
                    "{:<13} {:<34} {}",
                    f.name,
                    f.commands.join(","),
                    f.about
                )?;
            }
            Ok(())
        }
        "bench" => bench::run(rest, io),
        name => {
            let format = formats::find(name)
                .ok_or_else(|| CliError::usage(format!("unknown format {name}\n{USAGE}")))?;
            match rest.split_first() {
                Some((cmd, cmd_args)) if format.commands.contains(&cmd.as_str()) => {
                    (format.run)(cmd, cmd_args, io)
                }
                _ => (format.run)(format.commands[0], rest, io),
            }
        }
    }
}
