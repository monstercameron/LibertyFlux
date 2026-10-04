//! Print a JSON catalog of every cutscene in the game (to stdout only).
//!
//! ```text
//! LIBERTYFLUX_GAME_DIR="C:\path\to\Grand Theft Auto IV" cargo run -p lf-cutscene --example lf_cutscene_catalog
//! ```
//!
//! Reads the game folder; never writes to it. The archive key is located in
//! the owner's installed executable at run time and kept in memory only.

use std::path::PathBuf;

use lf_cutscene::catalog;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir: PathBuf = std::env::var_os("LIBERTYFLUX_GAME_DIR")
        .map(PathBuf::from)
        .ok_or("set LIBERTYFLUX_GAME_DIR to the game folder")?;
    let exe = dir.join("GTAIV").join("GTAIV.exe");
    let key = lf_archive::crypto::load_key_from_exe(&exe)?;
    let mut parsed = Vec::new();
    for path in catalog::find_archives(&dir, "cuts.img") {
        parsed.extend(catalog::parse_cuts_in_archive(&path, &key)?);
    }
    let rows = catalog::catalog_rows(&parsed);
    print!("{}", catalog::catalog_json(&rows));
    Ok(())
}
