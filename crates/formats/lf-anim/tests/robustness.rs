//! Fuzz-style robustness: truncated and corrupted inputs must return
//! errors, never panic. Added by the workspace assembly lane; needs no
//! game files.

use std::io::Write;
use std::panic::{AssertUnwindSafe, catch_unwind};

use flate2::Compression;
use flate2::write::ZlibEncoder;
use lf_anim::{AnimDictionary, ChannelData};

/// Synthetic byte patterns: empty, zeros, ones, runs and sweeps. Nothing
/// here comes from the game.
fn patterns() -> Vec<Vec<u8>> {
    vec![
        Vec::new(),
        vec![0u8; 512],
        vec![0xFFu8; 512],
        vec![0x41u8; 512],
        (0..512u32).map(|i| (i & 0xFF) as u8).collect(),
        (0..512u32)
            .map(|i| (i.wrapping_mul(31).wrapping_add(7) & 0xFF) as u8)
            .collect(),
    ]
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

/// Drive the read API past parsing: clips, tracks, channels, quats and
/// the validation rules.
fn drive(bytes: &[u8]) {
    if let Ok(dict) = AnimDictionary::parse_bytes(bytes) {
        let _ = dict.len();
        let _ = dict.pool();
        for clip in dict.clips() {
            let _ = clip.validate_frame_duration();
            let _ = clip.validate_hash();
            let _ = clip.static_pose();
            for track in clip.tracks() {
                let ch = track.channel();
                if let ChannelData::StaticQuat(q) = ch.data() {
                    let _ = q.norm();
                    let _ = q.is_unit(0.01);
                }
            }
        }
        if let Some(pool) = dict.pool() {
            for track in pool {
                for ch in track.channels() {
                    let _ = ch.tag();
                }
            }
        }
    }
}

fn w32(sys: &mut [u8], off: usize, v: u32) {
    sys[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

fn w16(sys: &mut [u8], off: usize, v: u16) {
    sys[off..off + 2].copy_from_slice(&v.to_le_bytes());
}

fn wf32(sys: &mut [u8], off: usize, v: f32) {
    w32(sys, off, v.to_bits());
}

/// Wrap a system segment in an RSC5 file (kind 1, no graphics segment).
fn rsc(sys: &[u8]) -> Vec<u8> {
    assert!(sys.len().is_multiple_of(256));
    let flags: u32 = u32::try_from(sys.len() / 256).expect("fixture fits");
    let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
    enc.write_all(sys).unwrap();
    let payload = enc.finish().unwrap();
    let mut file = Vec::new();
    file.extend_from_slice(&0x0543_5352u32.to_le_bytes());
    file.extend_from_slice(&1u32.to_le_bytes());
    file.extend_from_slice(&flags.to_le_bytes());
    file.extend_from_slice(&payload);
    file
}

/// Minimal one-clip dictionary; the track channel is a static unit quat.
/// Hand-built; no game bytes.
fn one_clip() -> Vec<u8> {
    let mut sys = vec![0u8; 512];
    w32(&mut sys, 0x00, 0x69_5374);
    w32(&mut sys, 0x04, 0x5000_0300);
    w32(&mut sys, 0x0C, 1);
    w32(&mut sys, 0x10, 0x5000_0040);
    w32(&mut sys, 0x14, 0x0001_0001);
    w32(&mut sys, 0x18, 0x5000_0050);
    w32(&mut sys, 0x1C, 0x0001_0001);
    w32(&mut sys, 0x40, lf_anim::clip_hash("hop"));
    w32(&mut sys, 0x50, 0x5000_0060);
    w32(&mut sys, 0x60, 0x6a_ef64);
    w32(&mut sys, 0x64, 0x01f1_0001);
    w16(&mut sys, 0x68, 11);
    w16(&mut sys, 0x6A, 15);
    wf32(&mut sys, 0x6C, 10.0 / 30.0);
    w32(&mut sys, 0x70, 0x1234_5678);
    w32(&mut sys, 0x74, 0x5000_0200);
    w32(&mut sys, 0x78, 0x0001_0001);
    w32(&mut sys, 0x7C, 0x5000_0100);
    w32(&mut sys, 0x80, 0x5000_0090);
    w32(&mut sys, 0x84, 0x0001_0001);
    w32(&mut sys, 0x88, 0x142);
    w32(&mut sys, 0x90, 0x5000_00A0);
    w32(&mut sys, 0xA0, 0x380);
    w32(&mut sys, 0xA4, 0x0F);
    w32(&mut sys, 0xA8, 0x5000_00B0);
    w32(&mut sys, 0xAC, 0x0001_0001);
    w32(&mut sys, 0xB0, 0x5000_00C0);
    w32(&mut sys, 0xC0, 0x380);
    w32(&mut sys, 0xC4, 0x5000_00E0);
    w32(&mut sys, 0xD4, 1);
    w32(&mut sys, 0xE0, 0x6a_f86c);
    w32(&mut sys, 0xE4, 0x900);
    w32(&mut sys, 0xE8, 0x5000_00F0);
    w32(&mut sys, 0xEC, 0);
    wf32(&mut sys, 0xF0, 0.0);
    wf32(&mut sys, 0xF4, 0.0);
    wf32(&mut sys, 0xF8, 0.0);
    wf32(&mut sys, 0xFC, 1.0);
    sys[0x100..0x100 + 15].copy_from_slice(b"pack:/hop.anim\0");
    rsc(&sys)
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

/// Byte flips inside a valid dictionary must not panic either; this
/// reaches the deep clip/track/channel paths the sweeps above cannot.
#[test]
fn corrupted_valid_dictionary_never_panics() {
    let good = one_clip();
    assert!(AnimDictionary::parse_bytes(&good).is_ok());
    for pos in (0..good.len()).step_by(11) {
        for mask in [0xFFu8, 0x01, 0x80] {
            let mut bad = good.clone();
            bad[pos] ^= mask;
            check_no_panic(&format!("byte {pos} xor {mask:#04X}"), || drive(&bad));
        }
    }
}

#[test]
fn empty_input_returns_error() {
    assert!(AnimDictionary::parse_bytes(&[]).is_err());
}
