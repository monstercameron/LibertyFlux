//! Print a JSON census of every effect file under a game folder.
//!
//! Usage: `lf_effects_census <game-dir>`. Writes JSON to stdout only; never writes
//! game content anywhere. Counts and bank shapes only, no name lists.

use std::path::{Path, PathBuf};

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.is_file() {
            out.push(path);
        }
    }
}

fn json_str(s: &str) -> String {
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

fn main() {
    let dir = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: lf_effects_census <game-dir>");
        std::process::exit(2);
    });
    let mut files = Vec::new();
    collect(Path::new(&dir), &mut files);
    files.sort();

    let root = Path::new(&dir);
    let mut wpfl_json = Vec::new();
    let mut dat_json = Vec::new();
    let mut xml_json = Vec::new();
    for path in &files {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if ext == "wpfl" {
            let bytes = std::fs::read(path).unwrap();
            let pkg = lf_effects::WpflFile::parse(&bytes).unwrap();
            let banks: Vec<String> = pkg
                .banks()
                .iter()
                .map(|b| {
                    let resolved = b
                        .entries()
                        .iter()
                        .filter(|e| pkg.name_of(e.hash).is_some())
                        .count();
                    format!(
                        "{{\"slot\":{},\"vtable\":\"{:#010x}\",\"entries\":{},\"resolved\":{}}}",
                        b.slot,
                        b.vtable,
                        b.len(),
                        resolved
                    )
                })
                .collect();
            wpfl_json.push(format!(
                "{{\"file\":{},\"kind\":\"{:#04x}\",\"banks\":[{}]}}",
                json_str(name),
                pkg.kind(),
                banks.join(",")
            ));
        } else if ext == "dat" && name.ends_with("Fx.dat") {
            let text = std::fs::read_to_string(path).unwrap();
            let file = lf_effects::FxFile::parse(&text).unwrap();
            let tables: Vec<String> = file
                .tables
                .iter()
                .map(|t| {
                    format!(
                        "{{\"name\":{},\"rows\":{},\"width\":{}}}",
                        json_str(&t.name),
                        t.len(),
                        t.width().unwrap_or(0)
                    )
                })
                .collect();
            let rel = path
                .strip_prefix(root)
                .unwrap_or(path)
                .to_string_lossy()
                .replace('\\', "/");
            dat_json.push(format!(
                "{{\"file\":{},\"version\":{},\"tables\":[{}]}}",
                json_str(&rel),
                json_str(&file.version),
                tables.join(",")
            ));
        } else if ext == "xml"
            && matches!(
                name,
                "gtaRainEmitter.xml"
                    | "gtaRainRender.xml"
                    | "gtaStormEmitter.xml"
                    | "gtaStormRender.xml"
            )
        {
            let text = std::fs::read_to_string(path).unwrap();
            let file = lf_effects::emitter::parse(&text).unwrap();
            let rel = path
                .strip_prefix(root)
                .unwrap_or(path)
                .to_string_lossy()
                .replace('\\', "/");
            xml_json.push(format!(
                "{{\"file\":{},\"root\":{},\"props\":{}}}",
                json_str(&rel),
                json_str(&file.root),
                file.props.len()
            ));
        }
    }
    println!(
        "{{\"wpfl\":[{}],\"dat\":[{}],\"xml\":[{}]}}",
        wpfl_json.join(","),
        dat_json.join(","),
        xml_json.join(",")
    );
}
