//! Fuzz-style robustness: truncated and corrupted inputs must return
//! errors, never panic. Added by the workspace assembly lane; needs no
//! game files.

use lf_archive::Archive;
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
    [&b"RPF2"[..], &b"RPF3"[..], &b"\x52\x2A\x4E\xA9"[..]].to_vec()
}

use std::io::Cursor;

#[test]
fn truncated_inputs_never_panic() {
    for base in patterns(&extra()).iter() {
        for len in lengths(base.len()) {
            let b = &base[..len];
            check_no_panic(&format!("detect prefix {len}"), || {
                let _ = lf_archive::detect(b, None);
            });
            check_no_panic(&format!("open prefix {len}"), || {
                let _ = lf_archive::open(&mut Cursor::new(b), None);
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
                    check_no_panic(&format!("corrupt detect prefix "), || {
                        let _ = lf_archive::detect(b, None);
                    });
                    check_no_panic(&format!("corrupt open prefix "), || {
                        let _ = lf_archive::open(&mut Cursor::new(b), None);
                    });
                });
            }
        }
    }
}

#[test]
fn empty_input_errors_or_skips() {
    assert!(lf_archive::open(&mut Cursor::new(&[]), None).is_err());
    assert_eq!(lf_archive::detect(&[], None), None);
}

#[test]
fn out_of_range_entry_index_errors() {
    // Minimal IMG table with zero entries: magic, version 3, count 0,
    // table size 0, record size 16, unknown 0.
    let mut header = Vec::new();
    header.extend_from_slice(&lf_archive::img::IMG_MAGIC.to_le_bytes());
    header.extend_from_slice(&lf_archive::img::IMG_VERSION.to_le_bytes());
    header.extend_from_slice(&0u32.to_le_bytes());
    header.extend_from_slice(&0u32.to_le_bytes());
    header.extend_from_slice(&16u16.to_le_bytes());
    header.extend_from_slice(&0u16.to_le_bytes());
    let archive = lf_archive::img::ImgArchive::open(&mut Cursor::new(&header), None)
        .expect("empty table opens");
    assert_eq!(archive.entries().len(), 0);
    let r = archive.read_raw(&mut Cursor::new(&header), 0, None);
    assert!(r.is_err(), "index 0 of empty table must error, not panic");
    let r = archive.read_raw(&mut Cursor::new(&header), usize::MAX, None);
    assert!(r.is_err(), "huge index must error, not panic");
}
