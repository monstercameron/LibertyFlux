//! Fuzz-style robustness: truncated and corrupted inputs must return
//! errors, never panic. Added by the workspace assembly lane; needs no
//! game files.

use std::panic::{AssertUnwindSafe, catch_unwind};

/// Synthetic byte patterns: empty, zeros, ones, runs, sweeps and
/// magic-prefixed shapes. Nothing here comes from the game.
fn patterns(extra: &[&[u8]]) -> Vec<Vec<u8>> {
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
    for e in extra {
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

fn extra() -> Vec<&'static [u8]> {
    [].to_vec()
}

#[test]
fn truncated_inputs_never_panic() {
    for base in &patterns(&extra()) {
        for len in lengths(base.len()) {
            let b = &base[..len];
            check_no_panic(&format!("load prefix {len}"), || {
                let _ = lf_sco::container::load(b, None);
            });
            check_no_panic(&format!("decode_all prefix {len}"), || {
                let _ = lf_sco::isa::decode_all(b);
            });
            for off in [0, 1, len / 2, len.saturating_sub(1), len, len + 1] {
                check_no_panic(&format!("decode_one prefix {len} off {off}"), || {
                    let _ = lf_sco::isa::decode_one(b, off);
                });
            }
        }
    }
}

#[test]
fn corrupted_inputs_never_panic() {
    for (pi, base) in patterns(&extra()).iter().enumerate() {
        if base.is_empty() {
            continue;
        }
        for pos in (0..base.len()).step_by(37) {
            for mask in [0xFFu8, 0x01, 0x80] {
                let mut bad = base.clone();
                bad[pos] ^= mask;
                let what = format!("pattern {pi} byte {pos} xor {mask:#04X}");
                check_no_panic(&what, || {
                    let b = &bad;
                    check_no_panic("corrupt load prefix ", || {
                        let _ = lf_sco::container::load(b, None);
                    });
                    check_no_panic("corrupt decode_all prefix ", || {
                        let _ = lf_sco::isa::decode_all(b);
                    });
                    for off in [0, bad.len() / 2, bad.len()] {
                        check_no_panic(&format!("corrupt decode_one prefix off {off}"), || {
                            let _ = lf_sco::isa::decode_one(b, off);
                        });
                    }
                });
            }
        }
    }
}

#[test]
fn empty_input_returns_error() {
    assert!(lf_sco::container::load(&[], None).is_err());
    assert_eq!(lf_sco::isa::decode_all(&[]).expect("empty decodes"), vec![]);
    assert!(lf_sco::isa::decode_one(&[], 0).is_err());
    assert!(lf_sco::isa::decode_one(&[0u8; 8], 99).is_err());
}
