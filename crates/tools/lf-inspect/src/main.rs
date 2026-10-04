//! `lf-inspect` binary: see the library documentation for the commands.

use std::io::Write;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let mut out = stdout.lock();
    let mut err = stderr.lock();
    let result = lf_inspect::run(
        &args,
        &mut lf_inspect::Io {
            out: &mut out,
            err: &mut err,
        },
    );
    let _ = out.flush();
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            if !e.message.is_empty() {
                let _ = writeln!(err, "{}", e.message);
            }
            ExitCode::from(e.code)
        }
    }
}
