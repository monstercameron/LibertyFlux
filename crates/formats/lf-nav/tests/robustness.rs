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
        (0..512u32).map(|i| (i.wrapping_mul(31).wrapping_add(7) & 0xFF) as u8).collect(),
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
    for base in patterns(&extra()).iter() {
        for len in lengths(base.len()) {
            let b = &base[..len];
                        check_no_panic(&format!("nod prefix {len}"), || {
                            let _ = lf_nav::nod::Nod::parse(b);
                        });
                        check_no_panic(&format!("wnv prefix {len}"), || {
                            let _ = lf_nav::wnv::Tile::parse(b);
                        });
                        check_no_panic(&format!("rsc prefix {len}"), || {
                            let _ = lf_nav::rsc::Resource::parse(b);
                        });
                        check_no_panic(&format!("ipl prefix {len}"), || {
                            let text = String::from_utf8_lossy(b);
                            let _ = lf_nav::ipl::IplPaths::parse(&text);
                        });
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
                check_no_panic(&format!("corrupt nod prefix "), || {
                let _ = lf_nav::nod::Nod::parse(b);
                });
                check_no_panic(&format!("corrupt wnv prefix "), || {
                let _ = lf_nav::wnv::Tile::parse(b);
                });
                check_no_panic(&format!("corrupt rsc prefix "), || {
                let _ = lf_nav::rsc::Resource::parse(b);
                });
                check_no_panic(&format!("corrupt ipl prefix "), || {
                let text = String::from_utf8_lossy(b);
                let _ = lf_nav::ipl::IplPaths::parse(&text);
                });
                });
            }
        }
    }
}

#[test]
fn empty_input_returns_error() {
    assert!(lf_nav::nod::Nod::parse(&[]).is_err());
    assert!(lf_nav::wnv::Tile::parse(&[]).is_err());
    assert!(lf_nav::rsc::Resource::parse(&[]).is_err());
}
