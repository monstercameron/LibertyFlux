//! Command-line tests for `lf-inspect` on generated inputs. Every input is
//! generated here or by `lf_inspect::synth`; no game files are needed.
//!
//! Set `LF_INSPECT_KEEP_FIXTURES=1` to keep the generated files (the test
//! prints where they are), for example to compare the output with an older
//! build of the tool.

use std::path::{Path, PathBuf};

use lf_inspect::{CliError, Io, run, synth};

/// Run the tool, returning (result, stdout, stderr).
fn inspect(args: &[&str]) -> (Result<(), CliError>, String, String) {
    let args: Vec<String> = args.iter().map(|s| (*s).to_string()).collect();
    let mut out = Vec::new();
    let mut err = Vec::new();
    let result = run(
        &args,
        &mut Io {
            out: &mut out,
            err: &mut err,
        },
    );
    (
        result,
        String::from_utf8(out).unwrap(),
        String::from_utf8(err).unwrap(),
    )
}

fn ok(args: &[&str]) -> String {
    let (r, out, err) = inspect(args);
    assert!(
        r.is_ok(),
        "{args:?} failed: {r:?}\nstdout:\n{out}\nstderr:\n{err}"
    );
    out
}

/// A fresh fixture directory, removed on drop unless asked to keep it.
struct Fixtures(PathBuf);

impl Fixtures {
    fn new(name: &str) -> Fixtures {
        let dir = std::env::temp_dir().join(format!("lf-inspect-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        Fixtures(dir)
    }
    fn write(&self, name: &str, bytes: &[u8]) -> String {
        let p = self.0.join(name);
        std::fs::write(&p, bytes).unwrap();
        p.to_string_lossy().into_owned()
    }
}

impl Drop for Fixtures {
    fn drop(&mut self) {
        if std::env::var_os("LF_INSPECT_KEEP_FIXTURES").is_some() {
            eprintln!("fixtures kept in {}", self.0.display());
        } else {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}

#[test]
fn lists_every_format() {
    let out = ok(&["formats"]);
    for name in [
        "archive",
        "resource",
        "texture",
        "model",
        "collision",
        "nav",
        "text",
        "gamedata",
        "mapdata",
        "sco",
        "shaderpack",
        "audio-config",
        "save",
        "anim",
        "audio-bank",
        "cutscene",
        "effects",
        "entity-meta",
    ] {
        assert!(
            out.lines().any(|l| l.starts_with(name)),
            "{name} missing from:\n{out}"
        );
    }
    assert!(ok(&["help"]).contains("usage: lf-inspect"));
}

#[test]
fn usage_errors_exit_2_and_parse_errors_exit_1() {
    let (r, _, _) = inspect(&[]);
    assert_eq!(r.unwrap_err().code, 2);
    let (r, _, _) = inspect(&["no-such-format", "x"]);
    assert_eq!(r.unwrap_err().code, 2);
    let (r, _, _) = inspect(&["texture", "summarize"]);
    assert_eq!(r.unwrap_err().code, 2);
    let fx = Fixtures::new("errors");
    let junk = fx.write("junk.bin", b"\xFF\xFE not a format at all");
    // (nav and gamedata fall back to text readers, which accept anything.)
    for format in [
        "archive",
        "resource",
        "texture",
        "model",
        "collision",
        "sco",
        "shaderpack",
        "audio-config",
        "save",
        "anim",
        "audio-bank",
        "effects",
    ] {
        let (r, _, _) = inspect(&[format, "summarize", &junk]);
        let code = r.expect_err(format).code;
        assert_eq!(code, 1, "{format}");
    }
    let (r, _, _) = inspect(&["save", "summarize", "/no/such/file"]);
    assert!(r.unwrap_err().message.starts_with("cannot read"));
}

#[test]
fn archive_commands() {
    let fx = Fixtures::new("archive");
    let rpf = fx.write("a.rpf", &synth::rpf2(3));
    let out = ok(&["archive", "summarize", &rpf]);
    assert!(
        out.starts_with(
            "format: RPF2\ntable encrypted: false\ncontent encrypted: false\nrecords: 4\n"
        ),
        "{out}"
    );
    assert!(out.contains("files: 3  dirs: 1  resource: 0  compressed: 0"));
    assert!(out.contains("/file2.bin"));
    // The format's default command is summarize.
    assert_eq!(ok(&["archive", &rpf]), out);
    let list = ok(&["archive", "list", &rpf]);
    assert_eq!(list.lines().count(), 4);
    let dump = ok(&["archive", "dump", &rpf]);
    assert!(dump.starts_with("{\"kind\": \"RPF2\""));
    assert_eq!(dump.matches("\"path\"").count(), 4);
    let img = fx.write("b.img", &synth::img(2));
    let out = ok(&["archive", "summarize", &img]);
    assert!(
        out.starts_with("format: IMG v3\ntable encrypted: false\nentries: 2\n"),
        "{out}"
    );
}

#[test]
fn binary_format_summaries() {
    let fx = Fixtures::new("binary");
    let rsc = fx.write("r.rsc", &synth::resource(1));
    let out = ok(&["resource", "summarize", &rsc]);
    assert!(out.contains("system:    4096 bytes\n"), "{out}");
    assert!(ok(&["resource", "dump", &rsc]).contains("\"system\": 4096"));

    let gxt = fx.write("american.gxt", &synth::gxt(5));
    let out = ok(&["text", "summarize", &gxt]);
    assert!(
        out.contains("kind: GXT version=4 bits-per-char=16\ntables: 1 entries: 5\n"),
        "{out}"
    );
    assert_eq!(ok(&["text", "list", &gxt]), "MAIN 5\n");

    let wpl = fx.write("m.wpl", &synth::wpl(7));
    let out = ok(&["mapdata", "summarize", &wpl]);
    assert!(
        out.contains("format: WPL (binary)\nrecords: 7\n  inst: 7\n"),
        "{out}"
    );

    let nod = fx.write("nodes1.nod", &synth::nod(10, 2));
    let out = ok(&["nav", "summarize", &nod]);
    assert!(
        out.contains(
            "nodes: 10 (car 10, isec 0), links: 20\npartition clean: true\nowned links: 20"
        ),
        "{out}"
    );

    let sco = fx.write("s.sco", &synth::sco(30));
    let out = ok(&["sco", "summarize", &sco, "--disasm", "3"]);
    assert!(out.contains("instructions: 30\n"), "{out}");
    assert!(out.contains("... (27 more)"), "{out}");
    assert_eq!(ok(&["sco", "disasm", &sco]).lines().count(), 30);

    let save = fx.write("SGTA401", &synth::save(32, 16));
    let out = ok(&["save", "summarize", &save]);
    assert!(out.contains("blocks: 32\n"), "{out}");
    assert!(out.contains("checksum at "), "{out}");
    assert!(out.contains("(match)"), "{out}");
    assert_eq!(ok(&["save", "list", &save]).lines().count(), 32);
}

#[test]
fn text_format_summaries() {
    let fx = Fixtures::new("text");
    let ide = fx.write("vehicles.ide", &synth::ide_text(4));
    let out = ok(&["gamedata", "summarize", &ide]);
    assert!(out.contains("parser: ide\n  cars: 4\n"), "{out}");
    assert!(out.contains("  car: car0 (car)\n"), "{out}");
    assert!(ok(&["gamedata", "dump", &ide]).contains("\"parser\": \"ide\""));

    let ide2 = fx.write(
        "map.ide",
        b"objs\nprop, txd, 100, 0, 0, -1, -1, -1, 1, 1, 1, 0, 0, 0, 2, null\nend\n",
    );
    let out = ok(&["mapdata", "summarize", &ide2]);
    assert!(
        out.contains("format: IDE\nrecords: 1\n  objs: 1\nerrors: 0\n"),
        "{out}"
    );
    // Unknown extensions warn on standard error and parse as a load list.
    let odd = fx.write("list.xyz", b"IDE common:/data/a.ide\n");
    let (r, out, err) = inspect(&["mapdata", "summarize", &odd]);
    assert!(r.is_ok());
    assert_eq!(err, "unknown extension; treating as load list\n");
    assert!(
        out.contains("format: load list\ndirectives: 1\n  IDE: 1\n"),
        "{out}"
    );

    let cut = fx.write(
        "a.cut",
        b"[CUTSCENE_HEADER]\n1 2\n[/CUTSCENE_HEADER]\n[SECTION_START]\n[ANIM]\nwad_a\n[/ANIM]\n[DURATION]\n1500.0\n[/DURATION]\n[SECTION_END]\n",
    );
    let out = ok(&["cutscene", "summarize", &cut]);
    assert!(
        out.contains("group 0: 1 sections, 1500 ms, 0 models, 0 subtitles\n"),
        "{out}"
    );
    assert!(out.contains("  anims: [\"wad_a\"]\n"), "{out}");
    assert_eq!(
        ok(&["cutscene", "list", &cut]),
        "0 0 anim=wad_a audio= ms=1500\n"
    );

    let fxdat = fx.write("sampleFx.dat", b"1.0\nT_START\na b\nc d\nT_END\n");
    let out = ok(&["effects", "summarize", &fxdat]);
    assert_eq!(
        out,
        "version: 1.0\ntables: 1\ntable T: rows=2 width=Some(2)\n"
    );
    let xml = fx.write(
        "e.xml",
        b"<?xml version=\"1.0\"?>\n<root>\n<pos x=\"1\"/>\n</root>\n",
    );
    assert_eq!(
        ok(&["effects", "summarize", &xml]),
        "root: root\nproperties: 1\npos: x=1\n"
    );

    let hud = fx.write("hud.dat", b"[HD]\nHUD_A 0.1,0.2 0.3,0.4 HUD_COLOUR_A 255\n");
    let out = ok(&["text", "summarize", &hud]);
    assert!(out.ends_with("kind: hud.dat\n  [HD] items: 1\n"), "{out}");
    let paths = fx.write(
        "paths.ipl",
        b"vnod\n1, 2, 3\n4, 5, 6\nend\nlink\n0, 1, 0, 1\nend\n",
    );
    let out = ok(&["nav", "summarize", &paths]);
    assert!(
        out.ends_with("ipl: 2 nodes, 1 links (1 valid), 0 skipped lines\n"),
        "{out}"
    );
}

#[test]
fn file_and_synthetic_benchmarks_run() {
    let fx = Fixtures::new("bench");
    let wpl = fx.write("b.wpl", &synth::wpl(50));
    let out = ok(&["bench", "mapdata", &wpl, "--iters", "2"]);
    assert!(out.starts_with("mapdata"), "{out}");
    assert!(out.contains("iters    2"), "{out}");
    assert!(out.trim_end().ends_with("items 50"), "{out}");
    let out = ok(&["bench", "synthetic", "--iters", "1"]);
    assert_eq!(out.lines().count(), 9, "{out}");
    let (r, _, _) = inspect(&["bench", "nope", &wpl]);
    assert_eq!(r.unwrap_err().code, 2);
    assert!(Path::new(&wpl).exists());
}
