//! Property and fuzz tests on generated audio containers. Every byte is
//! generated here (see `crates/formats/tests/support.rs`); the "audio" is
//! random samples. No game files are needed.
//!
//! - Property: random banks (stream table in shuffled record order, wave
//!   headers, trailers, 16-bit PCM and ADPCM payloads) and random streamed
//!   files (several channels and blocks, preface, channel waves, file
//!   table, per-block channel infos and seek tables) parse back to the same
//!   fields, and PCM channels decode to exactly the samples written; ADPCM
//!   channels decode to the declared sample count.
//! - Fuzz: mutated banks, streamed files and configuration headers never
//!   panic anywhere in the API, decoding included.

// Fixture builders narrow random words and lengths into smaller fields on
// purpose, and the property checks compare floats that were written
// bit-exactly, so these pedantic lints do not apply here.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::float_cmp,
    clippy::format_push_string,
    clippy::format_collect,
    clippy::too_many_lines,
    clippy::type_complexity,
    clippy::many_single_char_names,
    clippy::similar_names
)]

#[path = "../../tests/support.rs"]
mod support;

use lf_audio_bank::bank::{Bank, CODEC_ADPCM, CODEC_PCM16};
use lf_audio_bank::dat::DatConfig;
use lf_audio_bank::streamed::Streamed;
use lf_audio_bank::{Container, detect};
use support::{Buf, Rng, fuzz};

/// Bytes covered by one seek entry of a streamed block.
const SEEK_BYTES: usize = 2048;
const BANK_HEADER: usize = 28;
const WAVE_HEADER: usize = 32;

#[derive(Debug, Clone)]
struct Sound {
    hash: u32,
    codec: u32,
    samples: Vec<i16>,
    /// ADPCM payload (two samples per byte) when the codec is ADPCM.
    adpcm: Vec<u8>,
    trailer: Vec<u8>,
}

fn random_sound(rng: &mut Rng) -> Sound {
    let n = rng.range(1, 600);
    if rng.chance(1, 2) {
        Sound {
            hash: rng.next_u32(),
            codec: CODEC_PCM16,
            samples: (0..n).map(|_| rng.next_u32() as i16).collect(),
            adpcm: Vec::new(),
            trailer: Vec::new(),
        }
    } else {
        let trailer_len = 21 + 3 * rng.below(3);
        Sound {
            hash: rng.next_u32(),
            codec: CODEC_ADPCM,
            samples: vec![0; n],
            adpcm: rng.bytes(n / 2),
            trailer: rng.bytes(trailer_len),
        }
    }
}

/// Write a bank. Records are stored in a shuffled order so table order and
/// record order differ, as in shipped banks.
fn write_bank(sounds: &[Sound], rng: &mut Rng) -> Vec<u8> {
    let n = sounds.len();
    let table_end = BANK_HEADER + n * 16;
    let mut order: Vec<usize> = (0..n).collect();
    for i in (1..n).rev() {
        order.swap(i, rng.below(i + 1));
    }
    let mut records = Buf::new();
    let mut info_off = vec![0usize; n];
    let mut data = Buf::new();
    let mut data_off = vec![0usize; n];
    for &i in &order {
        let s = &sounds[i];
        info_off[i] = records.len();
        data.align(SEEK_BYTES);
        data_off[i] = data.len();
        let (size, payload): (usize, Vec<u8>) = if s.codec == CODEC_PCM16 {
            (
                s.samples.len() * 2,
                s.samples.iter().flat_map(|v| v.to_le_bytes()).collect(),
            )
        } else {
            (s.adpcm.len(), s.adpcm.clone())
        };
        data.bytes(&payload);
        records
            .u32(data_off[i] as u32)
            .u32(0)
            .u32(s.hash)
            .u32(size as u32)
            .u32(s.samples.len() as u32)
            .u32(0)
            .u16(32000)
            .u16(0)
            .u32(s.codec)
            .bytes(&s.trailer);
    }
    let records_end = table_end + records.len();
    let base = records_end.next_multiple_of(SEEK_BYTES);
    let mut b = Buf::new();
    b.u32(BANK_HEADER as u32)
        .u32(0)
        .u32(records_end as u32)
        .u32(0)
        .u32(n as u32)
        .u32(0)
        .u32(base as u32);
    for (i, s) in sounds.iter().enumerate() {
        b.u32(info_off[i] as u32)
            .u32(0)
            .u32(s.hash)
            .u32((WAVE_HEADER + s.trailer.len()) as u32);
    }
    b.bytes(&records.0);
    b.put(base, &data.0);
    b.0
}

#[test]
fn bank_round_trip() {
    let mut rng = Rng::for_test("audio bank round trip");
    for _ in 0..60 {
        let sounds: Vec<Sound> = (0..rng.range(1, 6))
            .map(|_| random_sound(&mut rng))
            .collect();
        let bytes = write_bank(&sounds, &mut rng);
        let Ok(Container::Bank(bank)) = detect(&bytes) else {
            panic!("generated bank not detected as a bank");
        };
        assert_eq!(bank.stream_count() as usize, sounds.len());
        let entries = bank.entries().unwrap();
        for (e, s) in entries.iter().zip(&sounds) {
            assert_eq!(e.name_hash, s.hash);
            let info = bank.stream(e).expect("record reads");
            assert_eq!(info.codec, s.codec);
            assert_eq!(info.sample_count as usize, s.samples.len());
            assert_eq!(info.trailer, &s.trailer[..]);
            let decoded = bank.decode(&info).expect("generated sound decodes");
            if s.codec == CODEC_PCM16 {
                assert_eq!(decoded, s.samples);
            } else {
                assert_eq!(decoded.len(), s.samples.len());
                assert_eq!(bank.stream_data(&info).unwrap(), &s.adpcm[..]);
            }
        }
        assert!(bank.gap().iter().all(|&b| b == 0));
    }
}

#[derive(Debug, Clone)]
struct StreamSpec {
    codec: u32,
    /// Per block, per channel: seek entries (each 2048 payload bytes).
    entries: Vec<Vec<usize>>,
    /// Per channel: all decoded samples (PCM) across blocks.
    payload: Vec<Vec<Vec<u8>>>,
}

fn random_stream(rng: &mut Rng) -> StreamSpec {
    let channels = rng.range(1, 3);
    let blocks = rng.range(1, 3);
    let entries: Vec<Vec<usize>> = (0..blocks)
        .map(|_| (0..channels).map(|_| rng.range(1, 2)).collect())
        .collect();
    let payload = entries
        .iter()
        .map(|per_ch| per_ch.iter().map(|&e| rng.bytes(e * SEEK_BYTES)).collect())
        .collect();
    StreamSpec {
        codec: if rng.chance(1, 2) {
            CODEC_PCM16
        } else {
            CODEC_ADPCM
        },
        entries,
        payload,
    }
}

/// Samples one channel holds in one block.
fn block_samples(codec: u32, entries: usize) -> usize {
    if codec == CODEC_PCM16 {
        entries * SEEK_BYTES / 2
    } else {
        entries * SEEK_BYTES * 2
    }
}

fn write_stream(s: &StreamSpec) -> Vec<u8> {
    let blocks = s.entries.len();
    let channels = s.entries[0].len();
    let record_len = WAVE_HEADER + 24;
    let ch_table_off = 48usize;
    let aux_off = ch_table_off + channels * 16 + channels * record_len;
    let table_off = aux_off;
    let data_off = (table_off + blocks * 8).next_multiple_of(SEEK_BYTES);
    // Build each block, then size the chunk to the largest.
    let block_bytes: Vec<Vec<u8>> = (0..blocks)
        .map(|b| {
            let head = 24 + channels * 16;
            let mut blk = Buf::new();
            blk.u32(24)
                .u32(0)
                .u32(head as u32)
                .u32(0)
                .u32(head as u32)
                .u32(0);
            let mut start = 0;
            for c in 0..channels {
                let e = s.entries[b][c];
                blk.u32(start as u32)
                    .u32(e as u32)
                    .u32(0)
                    .u32(block_samples(s.codec, e) as u32);
                start += e;
            }
            let mut first = 0u32;
            for c in 0..channels {
                for _ in 0..s.entries[b][c] {
                    blk.u32(first).u32(first + 4095);
                    first += 4096;
                }
            }
            blk.align(SEEK_BYTES);
            for c in 0..channels {
                blk.bytes(&s.payload[b][c]);
            }
            blk.0
        })
        .collect();
    let chunk = block_bytes
        .iter()
        .map(Vec::len)
        .max()
        .unwrap()
        .next_multiple_of(SEEK_BYTES);
    let mut f = Buf::new();
    f.u32(table_off as u32)
        .u32(0)
        .u32(blocks as u32)
        .u32(chunk as u32)
        .u32(0)
        .u32(ch_table_off as u32)
        .u32(0)
        .u32(aux_off as u32)
        .u32(0)
        .u32(channels as u32)
        .u32(0)
        .u32(data_off as u32);
    for c in 0..channels {
        f.u32((c * record_len) as u32)
            .u32(0)
            .u32(0x100 + c as u32)
            .u32(record_len as u32);
    }
    for c in 0..channels {
        let total: usize = (0..blocks)
            .map(|b| block_samples(s.codec, s.entries[b][c]))
            .sum();
        let size: usize = (0..blocks).map(|b| s.entries[b][c] * SEEK_BYTES).sum();
        f.u32(0)
            .u32(0)
            .u32(0x100 + c as u32)
            .u32(size as u32)
            .u32(total as u32)
            .u32(0)
            .u16(32000)
            .u16(0)
            .u32(s.codec);
        f.bytes(&[0; 24]);
    }
    let mut first = 0u32;
    for b in 0..blocks {
        f.u32(first).u32(32000);
        first += block_samples(s.codec, s.entries[b][0]) as u32;
    }
    for (b, blk) in block_bytes.iter().enumerate() {
        f.put(data_off + b * chunk, blk);
    }
    let end = data_off + blocks * chunk;
    if f.len() < end {
        f.put(end - 1, &[0]);
    }
    f.0
}

#[test]
fn streamed_round_trip() {
    let mut rng = Rng::for_test("audio streamed round trip");
    for _ in 0..40 {
        let spec = random_stream(&mut rng);
        let bytes = write_stream(&spec);
        let Ok(Container::Streamed(s)) = detect(&bytes) else {
            panic!("generated streamed file not detected");
        };
        let channels = spec.entries[0].len();
        assert_eq!(s.block_count() as usize, spec.entries.len());
        assert_eq!(s.channel_count() as usize, channels);
        assert_eq!(s.preface().unwrap().len(), channels);
        let waves = s.channel_waves().expect("channel waves read");
        assert!(waves.iter().all(|w| w.codec == spec.codec));
        assert!(s.aux_region().unwrap().is_empty());
        assert_eq!(s.file_table().unwrap().len(), spec.entries.len());
        for b in 0..s.block_count() {
            let block = s.block(b).expect("block parses");
            for c in 0..channels {
                assert_eq!(
                    block.channel_data(c).unwrap(),
                    &spec.payload[b as usize][c][..]
                );
                assert_eq!(block.seeks[c].len(), spec.entries[b as usize][c]);
            }
        }
        for c in 0..channels {
            let decoded = s.decode_channel(c as u32).expect("channel decodes");
            assert_eq!(decoded.len() as u32, waves[c].sample_count);
            if spec.codec == CODEC_PCM16 {
                let want: Vec<i16> = spec
                    .payload
                    .iter()
                    .flat_map(|per_ch| {
                        per_ch[c]
                            .chunks(2)
                            .map(|p| i16::from_le_bytes([p[0], p[1]]))
                    })
                    .collect();
                assert_eq!(decoded, want);
            }
        }
    }
}

/// The whole API surface on one input.
fn exercise(bytes: &[u8]) {
    match detect(bytes) {
        Ok(Container::Bank(bank)) => drive_bank(&bank),
        Ok(Container::Streamed(s)) => drive_streamed(&s),
        Err(_) => {}
    }
    if let Ok(bank) = Bank::parse(bytes) {
        drive_bank(&bank);
    }
    if let Ok(d) = DatConfig::parse(bytes) {
        let _ = (
            d.objects().len(),
            d.names_region().len(),
            d.name_runs(3).len(),
        );
    }
}

fn drive_bank(bank: &Bank<'_>) {
    let _ = bank.gap();
    if let Ok(entries) = bank.entries() {
        for e in &entries {
            if let Ok(s) = bank.stream(e) {
                let _ = bank.stream_data(&s);
                let _ = bank.decode(&s);
            }
        }
    }
}

fn drive_streamed(s: &Streamed<'_>) {
    let _ = (
        s.preface(),
        s.channel_record_len(),
        s.channel_waves(),
        s.aux_region(),
    );
    if s.file_table().is_err() {
        return;
    }
    for b in 0..s.block_count().min(16) {
        if let Ok(block) = s.block(b) {
            for c in 0..block.channels.len().min(16) {
                let _ = block.channel_data(c);
                let _ = block.decode_channel(c, CODEC_ADPCM);
                let _ = block.decode_channel(c, CODEC_PCM16);
            }
        }
    }
    for c in 0..s.channel_count().min(16) {
        let _ = s.decode_channel(c);
    }
}

#[test]
fn fuzz_banks() {
    let mut rng = Rng::for_test("audio bank fuzz");
    let seeds: Vec<Vec<u8>> = (0..3)
        .map(|_| {
            let sounds: Vec<Sound> = (0..3).map(|_| random_sound(&mut rng)).collect();
            write_bank(&sounds, &mut rng)
        })
        .collect();
    fuzz("audio bank", &seeds, 1500, exercise);
}

#[test]
fn fuzz_streamed() {
    let mut rng = Rng::for_test("audio streamed fuzz");
    let seeds: Vec<Vec<u8>> = (0..2)
        .map(|_| write_stream(&random_stream(&mut rng)))
        .collect();
    fuzz("audio streamed", &seeds, 1000, exercise);
}

#[test]
fn fuzz_config_headers() {
    let mut seed = Buf::new();
    seed.u32(15)
        .u32(24)
        .u32(0)
        .u32(0)
        .bytes(b"\0\0\0\0\0\0\0\0\x05name1\x05name2");
    fuzz("audio dat config", &[seed.0], 2000, exercise);
}

#[test]
fn regression_records_end_past_base_has_an_empty_gap() {
    // A header whose records end lies past the data base: `gap` used to
    // slice backwards and panic.
    let mut b = Buf::new();
    b.u32(BANK_HEADER as u32)
        .u32(0)
        .u32(40)
        .u32(0)
        .u32(1)
        .u32(0)
        .u32(30);
    b.put(63, &[0]);
    let bank = Bank::parse(&b.0).expect("header checks pass");
    assert!(bank.gap().is_empty());
}
