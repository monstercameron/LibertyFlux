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
    [&b"RSC\x05"[..]].to_vec()
}

#[test]
fn truncated_inputs_never_panic() {
    for base in patterns(&extra()).iter() {
        for len in lengths(base.len()) {
            let b = &base[..len];
                        check_no_panic(&format!("rsc prefix {len}"), || {
                            let _ = lf_texture::Resource::parse(b);
                        });
                        check_no_panic(&format!("dictionary prefix {len}"), || {
                            let _ = lf_texture::Dictionary::parse(b);
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
                check_no_panic(&format!("corrupt rsc prefix "), || {
                let _ = lf_texture::Resource::parse(b);
                });
                check_no_panic(&format!("corrupt dictionary prefix "), || {
                let _ = lf_texture::Dictionary::parse(b);
                });
                });
            }
        }
    }
}

#[test]
fn empty_input_returns_error() {
    assert!(lf_texture::Resource::parse(&[]).is_err());
    assert!(lf_texture::Dictionary::parse(&[]).is_err());
}

#[test]
fn hostile_dimensions_never_panic() {
    use lf_texture::D3DFormat;
    let formats = [
        D3DFormat::Dxt1,
        D3DFormat::Dxt3,
        D3DFormat::Dxt5,
        D3DFormat::A8R8G8B8,
        D3DFormat::L8,
    ];
    let dims = [0u32, 1, 3, 4, 5, 255, 4096, 65_535, u32::MAX];
    let datas: Vec<Vec<u8>> = vec![
        Vec::new(),
        vec![0u8; 64],
        vec![0xFFu8; 64],
        (0..256u32).map(|i| (i & 0xFF) as u8).collect(),
    ];
    for f in formats {
        for w in dims {
            for h in dims {
                for data in &datas {
                    check_no_panic(&format!("decode {f:?} {w}x{h}"), || {
                        let _ = lf_texture::decode_to_rgba8(f, w, h, data);
                    });
                }
            }
        }
    }
}
