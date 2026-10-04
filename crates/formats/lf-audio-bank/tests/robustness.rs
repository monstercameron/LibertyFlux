//! Fuzz-style robustness: truncated and corrupted inputs must return
//! errors, never panic and never allocate without bound. Added by the
//! workspace assembly lane; needs no game files.

use std::panic::{AssertUnwindSafe, catch_unwind};

use lf_audio_bank::bank::{Bank, CODEC_ADPCM, CODEC_PCM16};
use lf_audio_bank::dat::DatConfig;
use lf_audio_bank::streamed::Streamed;

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

/// Drive the whole bank API surface past parsing.
fn drive_bank(bytes: &[u8]) {
    if let Ok(bank) = Bank::parse(bytes) {
        let _ = bank.gap();
        if let Ok(entries) = bank.entries() {
            for e in &entries {
                if let Ok(s) = bank.stream(e)
                    && bank.stream_data(&s).is_ok()
                {
                    let _ = bank.decode(&s);
                }
            }
        }
    }
}

/// Drive the whole streamed API surface past parsing. Block and channel
/// loops run only after the file table validates the block count, so
/// hostile counts cannot turn the loops into a hang.
fn drive_streamed(bytes: &[u8]) {
    if let Ok(s) = Streamed::parse(bytes) {
        let _ = s.preface();
        let _ = s.channel_record_len();
        let _ = s.channel_waves();
        let _ = s.aux_region();
        if s.file_table().is_err() {
            return;
        }
        for b in 0..s.block_count() {
            if let Ok(block) = s.block(b) {
                for c in 0..block.channels.len() {
                    if block.channel_data(c).is_ok() {
                        let _ = block.decode_channel(c, CODEC_ADPCM);
                        let _ = block.decode_channel(c, CODEC_PCM16);
                    }
                }
            }
        }
        for c in 0..s.channel_count() {
            let _ = s.decode_channel(c);
        }
    }
}

/// Drive the whole config API surface past parsing.
fn drive_dat(bytes: &[u8]) {
    if let Ok(d) = DatConfig::parse(bytes) {
        let _ = d.objects();
        let _ = d.names_region();
        let _ = d.name_runs(1);
        let _ = d.name_runs(4);
    }
}

#[test]
fn truncated_inputs_never_panic() {
    for base in &patterns(&[]) {
        for len in lengths(base.len()) {
            let b = &base[..len];
            check_no_panic(&format!("bank prefix {len}"), || drive_bank(b));
            check_no_panic(&format!("streamed prefix {len}"), || drive_streamed(b));
            check_no_panic(&format!("dat prefix {len}"), || drive_dat(b));
            check_no_panic(&format!("detect prefix {len}"), || {
                let _ = lf_audio_bank::detect(b);
            });
            check_no_panic(&format!("adpcm prefix {len}"), || {
                let _ = lf_audio_bank::adpcm::decode_mono(b, b.len() * 2);
            });
        }
    }
}

#[test]
fn corrupted_inputs_never_panic() {
    for (pi, base) in patterns(&[]).iter().enumerate() {
        if base.is_empty() {
            continue;
        }
        for pos in (0..base.len()).step_by(37) {
            for mask in [0xFFu8, 0x01, 0x80] {
                let mut bad = base.clone();
                bad[pos] ^= mask;
                let what = format!("pattern {pi} byte {pos} xor {mask:#04X}");
                check_no_panic(&what, || {
                    drive_bank(&bad);
                    drive_streamed(&bad);
                    drive_dat(&bad);
                    let _ = lf_audio_bank::detect(&bad);
                });
            }
        }
    }
}

#[test]
fn empty_input_returns_error() {
    assert!(Bank::parse(&[]).is_err());
    assert!(Streamed::parse(&[]).is_err());
    assert!(DatConfig::parse(&[]).is_err());
    assert!(lf_audio_bank::detect(&[]).is_err());
    assert!(lf_audio_bank::adpcm::decode_mono(&[], 1).is_err());
    assert_eq!(
        lf_audio_bank::adpcm::decode_mono(&[], 0).unwrap(),
        Vec::<i16>::new()
    );
}

/// A bank claiming 4 billion streams must fail fast, not pre-allocate.
#[test]
fn hostile_bank_count_fails_fast() {
    let mut b = vec![0u8; 28];
    b[0..8].copy_from_slice(&28u64.to_le_bytes());
    b[16..20].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(Bank::parse(&b).is_err());
}

/// A streamed file claiming 4 billion blocks (or channels) must fail
/// fast in the table readers, not pre-allocate.
#[test]
fn hostile_streamed_counts_fail_fast() {
    // blocks = MAX, chunk = 0: the block layout still ends at the input.
    let mut b = vec![0u8; 48];
    b[8..12].copy_from_slice(&u32::MAX.to_le_bytes());
    b[36..40].copy_from_slice(&1u32.to_le_bytes()); // channels
    b[44..48].copy_from_slice(&48u32.to_le_bytes()); // data_off = len
    let s = Streamed::parse(&b).unwrap();
    assert!(s.file_table().is_err());
    // channels = MAX with a one-block layout.
    let mut c = vec![0u8; 48];
    c[8..12].copy_from_slice(&1u32.to_le_bytes());
    c[36..40].copy_from_slice(&u32::MAX.to_le_bytes());
    c[44..48].copy_from_slice(&48u32.to_le_bytes());
    let s = Streamed::parse(&c).unwrap();
    assert!(s.preface().is_err());
}

/// A channel claiming 4 billion samples must hit the policy cap.
#[test]
fn hostile_sample_count_hits_decode_cap() {
    let mut b = tiny_streamed();
    // Wave sample count lives at channel info + 16.
    b[64 + 16..64 + 20].copy_from_slice(&u32::MAX.to_le_bytes());
    let s = Streamed::parse(&b).unwrap();
    assert_eq!(
        s.decode_channel(0).unwrap_err().kind(),
        lf_audio_bank::ErrorKind::Invalid
    );
}

/// Minimal streamed file: 1 channel, 1 block, record length 48, one seek
/// entry covering 2048 bytes / 4096 samples. Hand-built; no game bytes.
fn tiny_streamed() -> Vec<u8> {
    let mut b = vec![0u8; 2048 + 4096];
    b[0..8].copy_from_slice(&112u64.to_le_bytes());
    b[8..12].copy_from_slice(&1u32.to_le_bytes());
    b[12..16].copy_from_slice(&4096u32.to_le_bytes());
    b[16..20].copy_from_slice(&0u32.to_le_bytes());
    b[20..28].copy_from_slice(&48u64.to_le_bytes());
    b[28..32].copy_from_slice(&112u32.to_le_bytes());
    b[36..40].copy_from_slice(&1u32.to_le_bytes());
    b[44..48].copy_from_slice(&2048u32.to_le_bytes());
    b[48..56].copy_from_slice(&0u64.to_le_bytes());
    b[56..60].copy_from_slice(&0x1111u32.to_le_bytes());
    b[60..64].copy_from_slice(&48u32.to_le_bytes());
    b[64 + 12..64 + 16].copy_from_slice(&2048u32.to_le_bytes());
    b[64 + 16..64 + 20].copy_from_slice(&4096u32.to_le_bytes());
    b[64 + 24..64 + 26].copy_from_slice(&32000u16.to_le_bytes());
    b[64 + 28..64 + 32].copy_from_slice(&CODEC_ADPCM.to_le_bytes());
    b[112..116].copy_from_slice(&0u32.to_le_bytes());
    b[116..120].copy_from_slice(&32000u32.to_le_bytes());
    b[2048..2056].copy_from_slice(&24u64.to_le_bytes());
    b[2056..2064].copy_from_slice(&56u64.to_le_bytes());
    b[2064..2072].copy_from_slice(&56u64.to_le_bytes());
    b[2072..2076].copy_from_slice(&0u32.to_le_bytes());
    b[2076..2080].copy_from_slice(&1u32.to_le_bytes());
    b[2080..2084].copy_from_slice(&0u32.to_le_bytes());
    b[2084..2088].copy_from_slice(&4096u32.to_le_bytes());
    b[2104..2108].copy_from_slice(&0u32.to_le_bytes());
    b[2108..2112].copy_from_slice(&4095u32.to_le_bytes());
    b
}
