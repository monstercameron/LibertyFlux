//! Shared command-line plumbing: the error type with its exit code, the
//! output pair every command writes to, and small argument helpers.

use std::fmt;
use std::io::Write;

/// Exit code for a usage mistake (wrong or missing arguments).
pub const EXIT_USAGE: u8 = 2;
/// Exit code for a failure while reading or parsing input.
pub const EXIT_FAILURE: u8 = 1;

/// A failed command: the message for standard error (empty when the
/// command already printed what it had to say) and the process exit code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliError {
    /// Process exit code: [`EXIT_USAGE`] or [`EXIT_FAILURE`].
    pub code: u8,
    /// Message for standard error, without a trailing newline.
    pub message: String,
}

impl CliError {
    /// A usage error (exit code 2).
    #[must_use]
    pub fn usage(message: impl Into<String>) -> CliError {
        CliError {
            code: EXIT_USAGE,
            message: message.into(),
        }
    }

    /// A runtime failure (exit code 1).
    #[must_use]
    pub fn failure(message: impl Into<String>) -> CliError {
        CliError {
            code: EXIT_FAILURE,
            message: message.into(),
        }
    }

    /// A failure whose explanation was already written to the output.
    #[must_use]
    pub fn silent(code: u8) -> CliError {
        CliError {
            code,
            message: String::new(),
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> CliError {
        CliError::failure(format!("cannot write output: {e}"))
    }
}

/// Result type of every command.
pub type CliResult = Result<(), CliError>;

/// Where a command writes: `out` for the report, `err` for warnings that
/// do not stop the command.
pub struct Io<'a> {
    /// Standard output (the report).
    pub out: &'a mut dyn Write,
    /// Standard error (warnings).
    pub err: &'a mut dyn Write,
}

/// Read a whole file, mapping failure to "cannot read <path>: <reason>".
///
/// # Errors
///
/// Returns a failure when the file cannot be read.
pub fn read_file(path: &str) -> Result<Vec<u8>, CliError> {
    std::fs::read(path).map_err(|e| CliError::failure(format!("cannot read {path}: {e}")))
}

/// Require exactly one positional argument (a file path).
///
/// # Errors
///
/// Returns a usage error showing `usage` when the count is wrong.
pub fn one_path<'a>(args: &'a [String], usage: &str) -> Result<&'a str, CliError> {
    match args {
        [path] => Ok(path),
        _ => Err(CliError::usage(format!("usage: {usage}"))),
    }
}

/// Load the archive key from the owner's executable (memory only).
///
/// # Errors
///
/// Returns a failure when the key cannot be located.
pub fn load_key(exe: &str) -> Result<lf_archive::Key, CliError> {
    lf_archive::crypto::load_key_from_exe(std::path::Path::new(exe))
        .map_err(|e| CliError::failure(format!("cannot locate archive key in {exe}: {e}")))
}

/// Minimal JSON string escaping for the JSON-ish outputs.
#[must_use]
pub fn json_str(s: &str) -> String {
    use std::fmt::Write as _;
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => {
                write!(out, "\\u{:04x}", c as u32).expect("write to String cannot fail");
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// True when `path` ends in the extension `ext` (without the dot),
/// compared case-insensitively.
#[must_use]
pub fn has_ext(path: &str, ext: &str) -> bool {
    std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case(ext))
}
