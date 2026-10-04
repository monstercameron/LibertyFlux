//! Print a summary of one `.cut` file (read-only, never writes).
//!
//! ```text
//! cargo run -p lf-cutscene --example lf_cutscene_summarize -- path/to/file.cut
//! ```

use std::collections::HashSet;

use lf_cutscene::cut::CutsceneFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: lf_cutscene_summarize <file.cut>")?;
    let bytes = std::fs::read(&path)?;
    println!("file: {path} ({} bytes)", bytes.len());
    let file = CutsceneFile::parse(&bytes)?;
    println!("cutscenes: {}", file.groups.len());
    println!("trailing slack: {} bytes", file.trailing_slack_len);
    for (gi, g) in file.groups.iter().enumerate() {
        let anims: HashSet<&str> = g
            .sections
            .iter()
            .flat_map(|s| s.anims.iter().map(String::as_str))
            .collect();
        let mut anims: Vec<&str> = anims.into_iter().collect();
        anims.sort_unstable();
        let audios: HashSet<&str> = g
            .sections
            .iter()
            .flat_map(|s| s.audios.iter().map(String::as_str))
            .collect();
        let mut audios: Vec<&str> = audios.into_iter().collect();
        audios.sort_unstable();
        println!(
            "group {gi}: {} sections, {:.0} ms, {} models, {} subtitles",
            g.sections.len(),
            g.duration_ms(),
            g.model_count(),
            g.texts.len()
        );
        println!("  header frames: {:?}", g.header_frames);
        println!("  anims: {anims:?}");
        println!("  audios: {audios:?}");
        println!("  flags: {:?}", g.flags);
    }
    for w in &file.warnings {
        println!("warning: line {} {:?}: {}", w.line, w.kind, w.message);
    }
    Ok(())
}
