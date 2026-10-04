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
            check_no_panic(&format!("gxt prefix {len}"), || {
                let _ = lf_text::GxtFile::parse(b);
            });
            check_no_panic(&format!("fonts prefix {len}"), || {
                let _ = lf_text::FontFile::parse(b);
            });
            check_no_panic(&format!("menus prefix {len}"), || {
                let _ = lf_text::MenuFile::parse(b);
            });
            check_no_panic(&format!("dat prefix {len}"), || {
                let _ = lf_text::FrontendLayout::parse(b);
            });
            check_no_panic(&format!("radio prefix {len}"), || {
                let _ = lf_text::RadioHudFile::parse(b);
                let _ = lf_text::RadioHud::parse(b);
            });
            check_no_panic(&format!("hud prefix {len}"), || {
                let _ = lf_text::HudFile::parse(b);
                let _ = lf_text::HudColours::parse(b);
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
                    check_no_panic("corrupt gxt prefix ", || {
                        let _ = lf_text::GxtFile::parse(b);
                    });
                    check_no_panic("corrupt fonts prefix ", || {
                        let _ = lf_text::FontFile::parse(b);
                    });
                    check_no_panic("corrupt menus prefix ", || {
                        let _ = lf_text::MenuFile::parse(b);
                    });
                    check_no_panic("corrupt dat prefix ", || {
                        let _ = lf_text::FrontendLayout::parse(b);
                    });
                    check_no_panic("corrupt radio prefix ", || {
                        let _ = lf_text::RadioHudFile::parse(b);
                        let _ = lf_text::RadioHud::parse(b);
                    });
                    check_no_panic("corrupt hud prefix ", || {
                        let _ = lf_text::HudFile::parse(b);
                        let _ = lf_text::HudColours::parse(b);
                    });
                });
            }
        }
    }
}

#[test]
fn empty_gxt_returns_error() {
    assert!(lf_text::GxtFile::parse(&[]).is_err());
}

#[test]
fn western_decoder_never_panics() {
    let units: Vec<u16> = (0..=u16::MAX).collect();
    check_no_panic("full u16 range", || {
        let _ = lf_text::decode_western_lossy(&units);
    });
}

#[test]
fn hostile_table_count_is_capped_not_allocated() {
    // GXT header with a directory size of u32::MAX and no directory
    // behind it. Must error on truncation, not pre-allocate.
    let mut buf = Vec::new();
    buf.extend_from_slice(&4u16.to_le_bytes()); // version
    buf.extend_from_slice(&16u16.to_le_bytes()); // bits per unit
    buf.extend_from_slice(b"TABL");
    buf.extend_from_slice(&0xFFFF_FFFCu32.to_le_bytes()); // dir size (/12)
    assert!(lf_text::GxtFile::parse(&buf).is_err());
}
