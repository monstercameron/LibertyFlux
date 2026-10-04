//! Property and fuzz tests on generated effect files. Every byte is
//! generated here (see `crates/formats/tests/support.rs`); all names are
//! invented. No game files are needed.
//!
//! - Property: random effect packages (banks in random root slots, entries
//!   keyed by [`jenkins_oat`] name hashes, records with vtable words, name
//!   strings for resolution) parse back to the same slots, hashes, vtables
//!   and resolved names; random `*Fx.dat` tables and emitter XML files parse
//!   back to the rows and properties written.
//! - Fuzz: mutated system segments (re-wrapped in a valid container),
//!   texts and XML never panic.

// Fixture builders narrow random words and lengths into smaller fields on
// purpose, and the property checks compare floats that were written
// bit-exactly, so these pedantic lints do not apply here.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::float_cmp,
    clippy::format_push_string,
    clippy::format_collect,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::many_single_char_names,
    clippy::similar_names
)]

#[path = "../../tests/support.rs"]
mod support;

use lf_effects::{FxFile, WpflFile, emitter, jenkins_oat};
use support::{Buf, Rng, fuzz, rsc5};

const SYS: u32 = 0x5000_0000;
const KINDS: [u32; 2] = [0x24, 0x1B];
const ROOT_SLOTS: usize = 8;

/// Banks by root slot: bank vtable, entries of (name, record vtable).
type PackageSpec = Vec<(usize, u32, Vec<(String, u32)>)>;

fn random_package(rng: &mut Rng) -> PackageSpec {
    let mut slots: Vec<usize> = (0..ROOT_SLOTS).filter(|_| rng.chance(1, 3)).collect();
    if slots.is_empty() {
        slots.push(rng.below(ROOT_SLOTS));
    }
    slots
        .into_iter()
        .map(|slot| {
            let entries = (0..rng.range(1, 6))
                .map(|i| {
                    (
                        format!("fx_{}_{slot}_{i}", rng.ident(2, 10).to_lowercase()),
                        0x0040_0000 | rng.below(0xFFFF) as u32,
                    )
                })
                .collect();
            (slot, 0x0040_1000 + slot as u32, entries)
        })
        .collect()
}

fn write_package(p: &PackageSpec) -> Vec<u8> {
    let mut sys = Buf::zeroed(64);
    let alloc = |sys: &mut Buf, len: usize| {
        sys.align(16);
        let at = sys.len();
        sys.put(at, &vec![0; len.max(1)]);
        at
    };
    for (slot, vtable, entries) in p {
        let n = entries.len() as u32;
        let hashes = alloc(&mut sys, entries.len() * 4);
        let ptrs = alloc(&mut sys, entries.len() * 4);
        for (i, (name, rec_vt)) in entries.iter().enumerate() {
            let rec = alloc(&mut sys, 16);
            sys.put_u32(rec, *rec_vt);
            let name_at = alloc(&mut sys, name.len() + 1);
            sys.put(name_at, name.as_bytes());
            sys.put_u32(hashes + 4 * i, jenkins_oat(name.as_bytes()))
                .put_u32(ptrs + 4 * i, SYS | rec as u32);
        }
        let bank = alloc(&mut sys, 32);
        sys.put_u32(bank, *vtable)
            .put_u32(bank + 12, 1)
            .put_u32(bank + 16, SYS | hashes as u32)
            .put_u32(bank + 20, n | (n << 16))
            .put_u32(bank + 24, SYS | ptrs as u32)
            .put_u32(bank + 28, n | (n << 16));
        sys.put_u32(slot * 4, SYS | bank as u32);
    }
    sys.0
}

#[test]
fn package_round_trip() {
    let mut rng = Rng::for_test("effects wpfl round trip");
    for case in 0..80 {
        let spec = random_package(&mut rng);
        let kind = KINDS[case % 2];
        let file = rsc5(kind, &write_package(&spec), &[0u8; 16]);
        let pkg = WpflFile::parse(&file).expect("generated package parses");
        assert_eq!(pkg.kind(), kind);
        assert_eq!(pkg.banks().len(), spec.len());
        let mut total = 0;
        for (bank, (slot, vtable, entries)) in pkg.banks().iter().zip(&spec) {
            assert_eq!((bank.slot, bank.vtable), (*slot, *vtable));
            assert_eq!(bank.len(), entries.len());
            for (e, (name, rec_vt)) in bank.entries().iter().zip(entries) {
                let h = jenkins_oat(name.as_bytes());
                assert_eq!(e.hash, h);
                assert_eq!(e.vtable, *rec_vt);
                assert_eq!(pkg.name_of(h), Some(name.as_str()));
                assert_eq!(bank.find(h).map(|f| f.vtable), Some(*rec_vt));
                assert_eq!(pkg.record_bytes(e, 4).unwrap(), &rec_vt.to_le_bytes());
            }
            total += entries.len();
        }
        assert_eq!(pkg.entry_count(), total);
        assert_eq!(pkg.resolved_count(), total);
        assert_eq!(pkg.entries().count(), total);
    }
}

fn random_fx(rng: &mut Rng) -> (String, String, Vec<(String, Vec<Vec<String>>)>) {
    let version = format!("{}.{}", rng.range(1, 3), rng.below(100));
    let tables: Vec<(String, Vec<Vec<String>>)> = (0..rng.range(1, 3))
        .map(|t| {
            let width = rng.range(1, 5);
            let rows = (0..rng.below(5))
                .map(|r| (0..width).map(|c| format!("v{t}_{r}_{c}")).collect())
                .collect();
            (format!("TABLE{t}"), rows)
        })
        .collect();
    let mut text = format!("{version}\r\n\r\n# TYPE\tNAME\r\n");
    for (name, rows) in &tables {
        text.push_str(&format!("{name}_START\r\n"));
        for r in rows {
            text.push_str(&r.join("\t\t"));
            text.push_str("\r\n");
        }
        text.push_str(&format!("{name}_END\r\n\r\n"));
    }
    (text, version, tables)
}

fn random_emitter(rng: &mut Rng) -> (String, String, Vec<(String, Vec<(String, String)>)>) {
    let root = format!("rage__{}", rng.ident(2, 12));
    let props: Vec<(String, Vec<(String, String)>)> = (0..rng.below(6))
        .map(|i| {
            let attrs = (0..rng.range(1, 3))
                .map(|a| (format!("a{a}"), format!("{}.000000", rng.below(100))))
                .collect();
            (format!("prop{i}"), attrs)
        })
        .collect();
    let mut text = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\n<{root}>\n");
    for (name, attrs) in &props {
        let a: String = attrs.iter().map(|(k, v)| format!(" {k}=\"{v}\"")).collect();
        text.push_str(&format!("\t<{name}{a}/>\n"));
    }
    text.push_str(&format!("</{root}>\n"));
    (text, root, props)
}

#[test]
fn text_formats_round_trip() {
    let mut rng = Rng::for_test("effects text round trip");
    for _ in 0..80 {
        let (text, version, tables) = random_fx(&mut rng);
        let file = FxFile::parse(&text).expect("generated Fx.dat parses");
        assert_eq!(file.version, version);
        assert_eq!(file.tables.len(), tables.len());
        for (t, (name, rows)) in file.tables.iter().zip(&tables) {
            assert_eq!(&t.name, name);
            assert_eq!(&t.rows, rows);
            assert_eq!(t.width(), rows.first().map(Vec::len));
        }
        let (text, root, props) = random_emitter(&mut rng);
        let file = emitter::parse(&text).expect("generated emitter parses");
        assert_eq!(file.root, root);
        let got: Vec<(String, Vec<(String, String)>)> = file
            .props
            .iter()
            .map(|p| (p.name.clone(), p.attrs.clone()))
            .collect();
        assert_eq!(got, props);
    }
}

#[test]
fn fuzz_packages() {
    let mut rng = Rng::for_test("effects wpfl fuzz");
    let seeds: Vec<Vec<u8>> = (0..4)
        .map(|_| write_package(&random_package(&mut rng)))
        .collect();
    fuzz("wpfl system segment", &seeds, 3000, |sys| {
        if let Ok(pkg) = WpflFile::parse(&rsc5(KINDS[0], sys, &[])) {
            for (_, e) in pkg.entries() {
                let _ = pkg.record_bytes(e, 64);
                let _ = pkg.name_of(e.hash);
            }
        }
    });
}

#[test]
fn fuzz_texts() {
    let mut rng = Rng::for_test("effects text fuzz");
    let seeds = vec![
        random_fx(&mut rng).0.into_bytes(),
        random_emitter(&mut rng).0.into_bytes(),
    ];
    fuzz("effects texts", &seeds, 4000, |b| {
        let text = String::from_utf8_lossy(b);
        if let Ok(f) = FxFile::parse(&text) {
            let _ = (f.row_count(), f.table("TABLE0"));
        }
        if let Ok(e) = emitter::parse(&text) {
            let _ = e.prop("prop0").and_then(|p| p.get("a0"));
        }
    });
}

#[test]
fn deeply_nested_emitter_xml_does_not_overflow() {
    // Nesting depth is attacker-controlled; the parser must fail or finish
    // without exhausting the stack.
    let depth = 20_000;
    let text = format!("<r>{}{}</r>", "<a>".repeat(depth), "</a>".repeat(depth));
    let _ = emitter::parse(&text);
}
