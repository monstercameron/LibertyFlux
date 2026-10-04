//! Fuzz-style robustness: truncated and corrupted inputs must return
//! errors, never panic. Added by the workspace assembly lane; needs no
//! game files.

use std::panic::{AssertUnwindSafe, catch_unwind};

use lf_effects::{FxFile, WpflFile, emitter};

/// Synthetic byte patterns: empty, zeros, ones, runs, sweeps and
/// shape-like prefixes. Nothing here comes from the game.
fn patterns() -> Vec<Vec<u8>> {
    let mut out = vec![
        Vec::new(),
        vec![0u8; 512],
        vec![0xFFu8; 512],
        vec![0x41u8; 512],
        (0..512u32).map(|i| (i & 0xFF) as u8).collect(),
        (0..512u32)
            .map(|i| (i.wrapping_mul(31).wrapping_add(7) & 0xFF) as u8)
            .collect(),
    ];
    for e in [
        &b"RSC\x05"[..],
        &b"1.00\nSTART_FX\n"[..],
        &b"<emitter name=\"x\">\n"[..],
    ] {
        let mut v = e.to_vec();
        v.resize(512, 0x00);
        out.push(v);
    }
    out
}

/// Prefix lengths to try: every short length, then a stride, then full.
fn lengths(full: usize) -> Vec<usize> {
    let mut out: Vec<usize> = (0..128.min(full + 1)).collect();
    let mut n = 128;
    while n < full {
        out.push(n);
        n += 7;
    }
    if !out.contains(&full) {
        out.push(full);
    }
    out
}

fn check_no_panic<F: FnOnce()>(what: &str, f: F) {
    let r = catch_unwind(AssertUnwindSafe(f));
    assert!(r.is_ok(), "panic on {what}");
}

/// Drive every reader past parsing.
fn drive(bytes: &[u8]) {
    if let Ok(pkg) = WpflFile::parse(bytes) {
        let _ = pkg.entry_count();
        let _ = pkg.resolved_count();
        for (_, entry) in pkg.entries() {
            let _ = pkg.name_of(entry.hash);
            let _ = pkg.record_bytes(entry, 4);
        }
    }
    // The text readers take `&str`; lossy conversion keeps the byte-level
    // corruption while satisfying the type.
    let text = String::from_utf8_lossy(bytes);
    if let Ok(fx) = FxFile::parse(&text) {
        for t in &fx.tables {
            let _ = t.rows.len();
        }
    }
    if let Ok(em) = emitter::parse(&text) {
        let _ = em.props.len();
    }
}

#[test]
fn truncated_inputs_never_panic() {
    for base in &patterns() {
        for len in lengths(base.len()) {
            let b = &base[..len];
            check_no_panic(&format!("prefix {len}"), || drive(b));
        }
    }
}

#[test]
fn corrupted_inputs_never_panic() {
    for (pi, base) in patterns().iter().enumerate() {
        if base.is_empty() {
            continue;
        }
        for pos in (0..base.len()).step_by(37) {
            for mask in [0xFFu8, 0x01, 0x80] {
                let mut bad = base.clone();
                bad[pos] ^= mask;
                check_no_panic(&format!("pattern {pi} byte {pos} xor {mask:#04X}"), || {
                    drive(&bad);
                });
            }
        }
    }
}

#[test]
fn empty_input_returns_error() {
    assert!(WpflFile::parse(&[]).is_err());
    assert!(FxFile::parse("").is_err());
    assert!(emitter::parse("").is_err());
}
