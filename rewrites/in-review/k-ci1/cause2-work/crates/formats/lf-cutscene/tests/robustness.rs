//! Fuzz-style robustness: truncated and corrupted inputs must return
//! errors (or warnings), never panic. Added by the workspace assembly
//! lane; needs no game files.

use std::panic::{AssertUnwindSafe, catch_unwind};

use lf_cutscene::cut::CutsceneFile;

/// Synthetic byte patterns: empty, zeros, ones, runs, sweeps and
/// tag-like shapes. Nothing here comes from the game.
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
    for e in [&b"[CUTSCENE_HEADER]\n1 2\n"[..], &b"\x00\xcd\x00\xff"[..]] {
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

/// Drive the read API past parsing: groups, sections, texts and models.
fn drive(bytes: &[u8]) {
    if let Ok(file) = CutsceneFile::parse(bytes) {
        for g in &file.groups {
            let _ = g.duration_ms();
            let _ = g.texts.len();
            for s in &g.sections {
                let _ = s.models.len();
                let _ = s.durations_ms.len();
            }
        }
        let _ = file.warnings.len();
        let _ = file.orphans.len();
        let _ = file.trailing_slack_len;
    }
}

/// Small valid file exercising the main tags. Hand-written; no game bytes.
fn small_valid() -> Vec<u8> {
    b"[CUTSCENE_HEADER]\n100 900\n[/CUTSCENE_HEADER]\n[SECTION_START]\n[MODELS]\n0 hero_0 hero_head_0 0\n[/MODELS]\n[DURATION]\n1500\n[/DURATION]\n[ANIM]\nhero_a\n[/ANIM]\n[SECTION_END]\n"
        .to_vec()
}

#[test]
fn truncated_inputs_never_panic() {
    for base in &patterns() {
        for len in lengths(base.len()) {
            let b = &base[..len];
            check_no_panic(&format!("prefix {len}"), || drive(b));
        }
    }
    let good = small_valid();
    for len in lengths(good.len()) {
        check_no_panic(&format!("valid prefix {len}"), || drive(&good[..len]));
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

/// Byte flips inside a valid file must not panic either.
#[test]
fn corrupted_valid_file_never_panics() {
    let good = small_valid();
    assert!(CutsceneFile::parse(&good).is_ok());
    for pos in (0..good.len()).step_by(3) {
        for mask in [0xFFu8, 0x01, 0x80, b'\n', b'[', b'\0'] {
            let mut bad = good.clone();
            bad[pos] ^= mask;
            check_no_panic(&format!("byte {pos} xor {mask:#04X}"), || drive(&bad));
        }
    }
}

/// Oversized inputs are a hard error, not a hang or an overflow.
#[test]
fn oversized_input_is_rejected() {
    let over = usize::try_from(lf_cutscene::cut::MAX_INPUT + 1).expect("limit fits");
    assert!(matches!(
        CutsceneFile::parse(&vec![b' '; over]),
        Err(lf_cutscene::Error::TooLarge { .. })
    ));
}
