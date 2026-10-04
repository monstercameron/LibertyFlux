//! `lf-viewer` command-line entry point; see the library docs and
//! `lf-viewer --help`.

use std::process::ExitCode;

use lf_viewer::cli::{CliError, run};

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let mut stdout = std::io::stdout();
    match run(&args, &mut stdout) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("lf-viewer: {e}");
            if matches!(e, CliError::Usage(_)) {
                ExitCode::from(2)
            } else {
                ExitCode::FAILURE
            }
        }
    }
}
