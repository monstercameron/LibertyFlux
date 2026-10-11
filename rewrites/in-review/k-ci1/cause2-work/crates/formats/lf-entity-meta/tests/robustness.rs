//! Fuzz-style robustness: truncated and corrupted inputs must return
//! errors, never panic. Added by the workspace assembly lane; needs no
//! game files.

use std::panic::{AssertUnwindSafe, catch_unwind};

/// Synthetic byte patterns: empty, zeros, ones, runs, sweeps and an
/// RSC-prefixed shape. Nothing here comes from the game.
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
    let mut v = b"RSC\x05".to_vec();
    v.resize(512, 0x00);
    out.push(v);
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

/// Drive every entry point past parsing.
fn drive(bytes: &[u8]) {
    if let Ok(lay) = lf_entity_meta::parse_vehicle(bytes) {
        let _ = lay.seat_count();
        let _ = lay.kind();
    }
    if let Ok(w) = lf_entity_meta::parse_weapon(bytes) {
        let _ = w.is_firearm();
        let _ = w.mounts().count();
    }
    // Same input as fragment and dictionary: still must not panic.
    let _ = lf_entity_meta::parse_ped(bytes, None);
    let _ = lf_entity_meta::parse_ped(bytes, Some(bytes));
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
    assert!(lf_entity_meta::parse_vehicle(&[]).is_err());
    assert!(lf_entity_meta::parse_weapon(&[]).is_err());
    assert!(lf_entity_meta::parse_ped(&[], None).is_err());
}
