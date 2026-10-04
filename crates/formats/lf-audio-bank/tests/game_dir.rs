//! Census over the real game data. Runs only when `LIBERTYFLUX_GAME_DIR`
//! points at the installed game folder (the one holding `GTAIV/`); otherwise
//! every test passes with a printed reason.
//!
//! The census parses every audio archive, every bank and streamed file inside
//! them, and every versioned audio config, then reports counts and every
//! distinct failure. When `LIBERTYFLUX_CENSUS_OUT` names a file, the census
//! numbers are also written there as JSON for later tools.

use lf_archive::Archive;
use lf_archive::rpf::RpfArchive;
use lf_audio_bank::{Container, bank, dat, streamed};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn game_dir() -> Option<PathBuf> {
    env::var_os("LIBERTYFLUX_GAME_DIR").map(PathBuf::from)
}

// Speech lookups without a version suffix use a different, unknown layout;
// failing them in the census is expected and asserted exactly.
const KNOWN_UNPARSED: &[&str] = &["SPEECH.DAT", "EP1_RADIO_SPEECH.DAT", "EP2_RADIO_SPEECH.DAT"];

/// FNV-1a 64 over little-endian i16 samples.
fn checksum(samples: &[i16]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for s in samples {
        for b in s.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x100_0000_01b3);
        }
    }
    h
}

/// Validation anchors verified sample-exact against vgmstream output: the
/// archive and entry hash, the bank stream index or streamed channel, the
/// expected sample count and rate, and the FNV-1a checksum of the decoded
/// PCM. Every anchor must be hit during the census.
struct Anchor {
    archive: &'static str,
    hash: u32,
    bank: bool,
    index: u32,
    samples: u32,
    rate: u16,
    checksum: u64,
}

const ANCHORS: &[Anchor] = &[
    Anchor {
        archive: "general.rpf",
        hash: 0x61b_65da,
        bank: true,
        index: 0,
        samples: 109_870,
        rate: 28_000,
        checksum: 0xe3d2_70f9_657a_0101,
    },
    Anchor {
        archive: "general.rpf",
        hash: 0x61b_65da,
        bank: true,
        index: 4,
        samples: 223_820,
        rate: 28_000,
        checksum: 0xdae6_9715_e646_3ffb,
    },
    Anchor {
        archive: "speech.rpf",
        hash: 0x6_bc78,
        bank: true,
        index: 0,
        samples: 50_851,
        rate: 24_000,
        checksum: 0xe889_14ad_4e53_c29a,
    },
    Anchor {
        archive: "radio_weather.rpf",
        hash: 0x684_0b59,
        bank: false,
        index: 0,
        samples: 409_651,
        rate: 32_000,
        checksum: 0xc98a_71ed_6c06_5ca3,
    },
    Anchor {
        archive: "radio_weather.rpf",
        hash: 0x684_0b59,
        bank: false,
        index: 1,
        samples: 409_651,
        rate: 32_000,
        checksum: 0x4f3_d458_7b84_ce30,
    },
    Anchor {
        archive: "radio_beat_95.rpf",
        hash: 0xe89a_e78c,
        bank: false,
        index: 0,
        samples: 78_281_738,
        rate: 32_000,
        checksum: 0xb40b_f8a0_590c_6e78,
    },
    Anchor {
        archive: "radio_beat_95.rpf",
        hash: 0xe89a_e78c,
        bank: false,
        index: 1,
        samples: 78_281_738,
        rate: 32_000,
        checksum: 0xafc3_4215_4622_f126,
    },
    Anchor {
        archive: "cutscenes.rpf",
        hash: 0xc6f1_1c39,
        bank: false,
        index: 0,
        samples: 292_267,
        rate: 32_000,
        checksum: 0x1f27_cb7b_5ba0_b31f,
    },
    Anchor {
        archive: "cutscenes.rpf",
        hash: 0xc6f1_1c39,
        bank: false,
        index: 1,
        samples: 292_267,
        rate: 32_000,
        checksum: 0xe089_83ef_39da_b5b7,
    },
    Anchor {
        archive: "cutscenes.rpf",
        hash: 0xc6f1_1c39,
        bank: false,
        index: 2,
        samples: 292_267,
        rate: 32_000,
        checksum: 0xfea7_3cb5_b783_e625,
    },
    Anchor {
        archive: "gps.rpf",
        hash: 0xe04_3d89,
        bank: true,
        index: 0,
        samples: 27_665,
        rate: 22_050,
        checksum: 0x3b7e_f568_85dd_62a1,
    },
    Anchor {
        archive: "radio_independence.rpf",
        hash: 0xb8_00bb,
        bank: false,
        index: 0,
        samples: 383_807,
        rate: 32_000,
        checksum: 0x4a5d_6ef1_cea0_31bc,
    },
    Anchor {
        archive: "radio_independence.rpf",
        hash: 0x37a6_580b,
        bank: false,
        index: 0,
        samples: 236_110,
        rate: 32_000,
        checksum: 0xb473_942f_1942_48b9,
    },
    Anchor {
        archive: "radio_independence.rpf",
        hash: 0x37a6_580b,
        bank: false,
        index: 1,
        samples: 236_110,
        rate: 32_000,
        checksum: 0x0ecb_9d65_b6d9_612e,
    },
];

#[derive(Default)]
struct Census {
    archives: u32,
    archive_files: u32,
    banks: u32,
    bank_streams: u32,
    bank_samples: u64,
    streamed_files: u32,
    streamed_channels: u32,
    streamed_samples: u64,
    configs: u32,
    config_names: u64,
    rates: BTreeSet<u16>,
    rate_streams: BTreeMap<u16, u64>,
    bank_codecs: BTreeMap<u32, u64>,
    stream_codecs: BTreeMap<u32, u64>,
    stream_channels: BTreeMap<u32, u64>,
    known_vorbis: u32,
    known_unparsed_configs: Vec<String>,
    h24_trailers: u64,
    skip_values: BTreeSet<u32>,
    bank_field14: BTreeSet<u32>,
    stream_flags: BTreeSet<u32>,
    trailer_heads: BTreeSet<u32>,
    failures: Vec<String>,
    archives_detail: Vec<ArchiveDetail>,
    anchor_hits: Vec<bool>,
}

// Eight plain parameters: bundling them into a struct would only move the
// field list elsewhere in this single-purpose test helper.
#[allow(clippy::too_many_arguments)]
fn check_anchor(
    census: &mut Census,
    archive: &str,
    hash: u32,
    bank: bool,
    index: u32,
    samples: u32,
    rate: u16,
    pcm: &[i16],
) {
    for (i, anchor) in ANCHORS.iter().enumerate() {
        if anchor.archive == archive
            && anchor.hash == hash
            && anchor.bank == bank
            && anchor.index == index
        {
            census.anchor_hits[i] = true;
            if samples != anchor.samples || rate != anchor.rate {
                note(
                    census,
                    format!(
                        "anchor {archive} {hash:#x} #{index}: header ({samples}, {rate}) vs ({}, {})",
                        anchor.samples, anchor.rate
                    ),
                );
            }
            let got = checksum(pcm);
            if got != anchor.checksum {
                note(
                    census,
                    format!(
                        "anchor {archive} {hash:#x} #{index}: PCM checksum {got:#x} vs {:#x}",
                        anchor.checksum
                    ),
                );
            }
        }
    }
}

struct ArchiveDetail {
    name: String,
    files: u32,
    banks: u32,
    streams: u32,
}

fn note(census: &mut Census, what: String) {
    if census.failures.len() < 200 {
        census.failures.push(what);
    }
}

// One linear census pass; splitting it would scatter the shared counters.
#[allow(clippy::too_many_lines)]
fn census_bank(census: &mut Census, archive: &str, hash: u32, data: &[u8], decode_all: bool) {
    let bank = match bank::Bank::parse(data) {
        Ok(b) => b,
        Err(e) => {
            note(census, format!("{archive} bank {hash:#x}: parse: {e}"));
            return;
        }
    };
    census.banks += 1;
    census.bank_field14.insert(bank.field_14());
    let entries = match bank.entries() {
        Ok(e) => e,
        Err(e) => {
            note(census, format!("{archive} bank {hash:#x}: table: {e}"));
            return;
        }
    };
    for entry in &entries {
        let stream = match bank.stream(entry) {
            Ok(s) => s,
            Err(e) => {
                note(
                    census,
                    format!("{archive} bank {hash:#x} stream {}: info: {e}", entry.index),
                );
                continue;
            }
        };
        census.bank_streams += 1;
        census.bank_samples += u64::from(stream.sample_count);
        census.rates.insert(stream.sample_rate);
        *census.rate_streams.entry(stream.sample_rate).or_insert(0) += 1;
        *census.bank_codecs.entry(stream.codec).or_insert(0) += 1;
        if stream.trailer.len() >= 4 {
            let head = u32::from_le_bytes(stream.trailer[0..4].try_into().unwrap());
            census.trailer_heads.insert(head);
        }
        // Trailer length law: ADPCM trailers hold a 21- or 24-byte header
        // plus 3 bytes per 2048-byte data sector; PCM sounds have no
        // trailer. What selects 24 is unknown.
        let sectors = stream.size.div_ceil(2048) as usize;
        let trailer_ok = match stream.codec {
            bank::CODEC_ADPCM => {
                stream.trailer.len() == 21 + 3 * sectors || stream.trailer.len() == 24 + 3 * sectors
            }
            bank::CODEC_PCM16 => stream.trailer.is_empty(),
            _ => true,
        };
        if stream.codec == bank::CODEC_ADPCM && stream.trailer.len() == 24 + 3 * sectors {
            census.h24_trailers += 1;
        }
        if !trailer_ok {
            note(
                census,
                format!(
                    "{archive} bank {hash:#x} stream {}: trailer {} vs size {}",
                    entry.index,
                    stream.trailer.len(),
                    stream.size
                ),
            );
        }
        let size_ok = match stream.codec {
            bank::CODEC_ADPCM => {
                u64::from(stream.size) == u64::from(stream.sample_count) / 2
                    || u64::from(stream.size) == u64::from(stream.sample_count.div_ceil(2))
            }
            bank::CODEC_PCM16 => u64::from(stream.size) == u64::from(stream.sample_count) * 2,
            _ => true, // unknown codecs have unknown size semantics
        };
        if !size_ok {
            note(
                census,
                format!(
                    "{archive} bank {hash:#x} stream {}: size {} vs samples {}",
                    entry.index, stream.size, stream.sample_count
                ),
            );
        }
        // The one Vorbis stream in the game: no decoder exists for it, so a
        // clean refusal is the expected behaviour, asserted exactly here.
        let is_known_vorbis = archive == "resident.rpf"
            && hash == 0x1e2_7c7e
            && entry.index == 24
            && stream.codec == 0x200;
        if decode_all {
            match bank.decode(&stream) {
                Ok(pcm) => {
                    assert_eq!(pcm.len(), stream.sample_count as usize);
                    check_anchor(
                        census,
                        archive,
                        hash,
                        true,
                        entry.index,
                        stream.sample_count,
                        stream.sample_rate,
                        &pcm,
                    );
                }
                Err(e) => {
                    if is_known_vorbis && e.kind() == lf_audio_bank::ErrorKind::Unsupported {
                        census.known_vorbis += 1;
                    } else {
                        note(
                            census,
                            format!(
                                "{archive} bank {hash:#x} stream {}: decode: {e}",
                                entry.index
                            ),
                        );
                    }
                }
            }
        }
    }
}

fn census_streamed(census: &mut Census, archive: &str, hash: u32, data: &[u8]) {
    let stream = match streamed::Streamed::parse(data) {
        Ok(s) => s,
        Err(e) => {
            note(census, format!("{archive} streamed {hash:#x}: parse: {e}"));
            return;
        }
    };
    census.streamed_files += 1;
    census.stream_flags.insert(stream.flags_28());
    *census
        .stream_channels
        .entry(stream.channel_count())
        .or_insert(0) += 1;
    let waves = match stream.channel_waves() {
        Ok(w) => w,
        Err(e) => {
            note(census, format!("{archive} streamed {hash:#x}: waves: {e}"));
            return;
        }
    };
    for wave in &waves {
        census.streamed_channels += 1;
        census.streamed_samples += u64::from(wave.sample_count);
        census.rates.insert(wave.sample_rate);
        *census.rate_streams.entry(wave.sample_rate).or_insert(0) += 1;
        *census.stream_codecs.entry(wave.codec).or_insert(0) += 1;
        if wave.trailer.len() >= 4 {
            let head = u32::from_le_bytes(wave.trailer[0..4].try_into().unwrap());
            census.trailer_heads.insert(head);
        }
    }
    if let Err(e) = stream.file_table() {
        note(
            census,
            format!("{archive} streamed {hash:#x}: file table: {e}"),
        );
    }
    let mut entries_per_channel = vec![0usize; stream.channel_count() as usize];
    for b in 0..stream.block_count() {
        match stream.block(b) {
            Ok(block) => {
                for (c, ch) in block.channels.iter().enumerate() {
                    entries_per_channel[c] += ch.entries as usize;
                    // The skip word's meaning is unknown; values are
                    // recorded, not failed.
                    census.skip_values.insert(ch.skip);
                    if block.channel_data(c).is_err() {
                        note(
                            census,
                            format!(
                                "{archive} streamed {hash:#x} block {b} ch {c}: payload out of range"
                            ),
                        );
                    }
                }
            }
            Err(e) => note(
                census,
                format!("{archive} streamed {hash:#x} block {b}: {e}"),
            ),
        }
    }
    for (c, wave) in waves.iter().enumerate() {
        let trailer_ok = match wave.codec {
            bank::CODEC_ADPCM => {
                wave.trailer.len() == 21 + 3 * entries_per_channel[c]
                    || wave.trailer.len() == 24 + 3 * entries_per_channel[c]
            }
            bank::CODEC_PCM16 => wave.trailer.is_empty(),
            _ => true,
        };
        if wave.codec == bank::CODEC_ADPCM && wave.trailer.len() == 24 + 3 * entries_per_channel[c]
        {
            census.h24_trailers += 1;
        }
        if !trailer_ok {
            note(
                census,
                format!(
                    "{archive} streamed {hash:#x} ch {c}: trailer {} vs {} entries",
                    wave.trailer.len(),
                    entries_per_channel[c]
                ),
            );
        }
    }
    let waves = stream.channel_waves().unwrap_or_default();
    for c in 0..stream.channel_count() {
        match stream.decode_channel(c) {
            Ok(pcm) => {
                let (samples, rate) = waves
                    .get(c as usize)
                    .map_or((0, 0), |w| (w.sample_count, w.sample_rate));
                check_anchor(census, archive, hash, false, c, samples, rate, &pcm);
            }
            Err(e) => note(
                census,
                format!("{archive} streamed {hash:#x} ch {c}: decode: {e}"),
            ),
        }
    }
}

fn audio_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for base in [
        "GTAIV/pc/audio/sfx",
        "GTAIV/TLAD/pc/audio/sfx",
        "GTAIV/TBoGT/pc/audio/sfx",
    ] {
        let dir = root.join(base);
        let mut names: Vec<_> = fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("list {}: {e}", dir.display()))
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|x| x == "rpf"))
            .collect();
        names.sort();
        out.extend(names);
    }
    out
}

fn config_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for base in [
        "GTAIV/pc/audio/config",
        "GTAIV/TLAD/pc/audio/config",
        "GTAIV/TBoGT/pc/audio/config",
    ] {
        let dir = root.join(base);
        let mut names: Vec<_> = fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("list {}: {e}", dir.display()))
            .map(|e| e.unwrap().path())
            .filter(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.contains(".dat") || n.contains(".DAT"))
            })
            .collect();
        names.sort();
        out.extend(names);
    }
    out
}

#[test]
// One linear census pass; splitting it would scatter the shared counters.
#[allow(clippy::too_many_lines)]
fn audio_census() {
    let Some(root) = game_dir() else {
        eprintln!("LIBERTYFLUX_GAME_DIR not set; skipping game census");
        return;
    };
    let key = lf_archive::crypto::load_key_from_exe(root.join("GTAIV").join("GTAIV.exe"))
        .expect("locate archive key");
    let mut census = Census {
        anchor_hits: vec![false; ANCHORS.len()],
        ..Default::default()
    };

    for archive_path in audio_files(&root) {
        let name = archive_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let buf = fs::read(&archive_path).unwrap();
        let mut reader = std::io::Cursor::new(&buf);
        let archive = match RpfArchive::open(&mut reader, Some(&key)) {
            Ok(a) => a,
            Err(e) => {
                note(&mut census, format!("{name}: archive parse: {e}"));
                continue;
            }
        };
        census.archives += 1;
        let before_banks = census.banks;
        let before_streams = census.bank_streams;
        let mut file_count = 0u32;
        for (index, entry) in archive.entries().iter().enumerate() {
            if !entry.is_file() {
                continue;
            }
            let hash = entry.hash.unwrap_or(0);
            census.archive_files += 1;
            file_count += 1;
            let data = match archive.read_file(&mut reader, index, Some(&key)) {
                Ok(d) => d,
                Err(e) => {
                    note(&mut census, format!("{name} file {hash:#x}: data: {e}"));
                    continue;
                }
            };
            match lf_audio_bank::detect(&data) {
                Ok(Container::Bank(_)) => {
                    census_bank(&mut census, &name, hash, &data, true);
                }
                Ok(Container::Streamed(_)) => {
                    census_streamed(&mut census, &name, hash, &data);
                }
                Err(e) => note(
                    &mut census,
                    format!("{name} file {hash:#x} size {}: detect: {e}", entry.size),
                ),
            }
        }
        census.archives_detail.push(ArchiveDetail {
            name,
            files: file_count,
            banks: census.banks - before_banks,
            streams: census.bank_streams - before_streams,
        });
    }

    for config_path in config_files(&root) {
        let file_name = config_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let buf = fs::read(&config_path).unwrap();
        match dat::DatConfig::parse(&buf) {
            Ok(d) => {
                census.configs += 1;
                let runs = d.name_runs(4);
                census.config_names += runs.len() as u64;
                println!(
                    "config {}: version={} names_off={} y={} z={} objects={} runs={}",
                    file_name,
                    d.version(),
                    d.names_off(),
                    d.field_y(),
                    d.field_z(),
                    d.objects().len(),
                    runs.len()
                );
            }
            Err(e) => {
                if KNOWN_UNPARSED.contains(&file_name.to_uppercase().as_str()) {
                    println!("config {file_name}: known-unknown layout ({e})");
                    census.known_unparsed_configs.push(file_name);
                } else {
                    note(
                        &mut census,
                        format!("{}: config parse: {e}", config_path.to_string_lossy()),
                    );
                }
            }
        }
    }

    println!(
        "archives={} files={}",
        census.archives, census.archive_files
    );
    println!(
        "banks={} streams={} samples={}",
        census.banks, census.bank_streams, census.bank_samples
    );
    println!(
        "streamed_files={} channels={} samples={}",
        census.streamed_files, census.streamed_channels, census.streamed_samples
    );
    println!(
        "configs={} config_name_runs={}",
        census.configs, census.config_names
    );
    println!("rates={:?}", census.rates);
    println!("rate_streams={:?}", census.rate_streams);
    println!(
        "bank_codecs={:?} stream_codecs={:?}",
        census.bank_codecs, census.stream_codecs
    );
    println!("stream_channels={:?}", census.stream_channels);
    println!(
        "bank_field14={:?} stream_flags={:?} trailer_heads={:?}",
        census.bank_field14, census.stream_flags, census.trailer_heads
    );
    println!(
        "known_vorbis={} known_unparsed_configs={:?}",
        census.known_vorbis, census.known_unparsed_configs
    );
    println!(
        "h24_trailers={} skip_values={:?}",
        census.h24_trailers, census.skip_values
    );
    for detail in &census.archives_detail {
        println!(
            "archive {}: files={} banks={} streams={}",
            detail.name, detail.files, detail.banks, detail.streams
        );
    }
    println!("failures={}", census.failures.len());
    for failure in census.failures.iter().take(100) {
        println!("FAILURE {failure}");
    }

    if let Some(out) = env::var_os("LIBERTYFLUX_CENSUS_OUT") {
        let rates: Vec<String> = census
            .rate_streams
            .iter()
            .map(|(rate, count)| format!("    \"{rate}\": {count}"))
            .collect();
        let channels: Vec<String> = census
            .stream_channels
            .iter()
            .map(|(ch, count)| format!("    \"{ch}\": {count}"))
            .collect();
        let json = format!(
            "{{\n  \"archives\": {},\n  \"archive_files\": {},\n  \"banks\": {},\n  \"bank_streams\": {},\n  \"bank_samples\": {},\n  \"streamed_files\": {},\n  \"streamed_channels\": {},\n  \"streamed_samples\": {},\n  \"configs\": {},\n  \"config_name_runs\": {},\n  \"rate_streams\": {{\n{}\n  }},\n  \"stream_channels\": {{\n{}\n  }},\n  \"failures\": {},\n  \"known_vorbis\": {},\n  \"known_unparsed_configs\": {}\n}}\n",
            census.archives,
            census.archive_files,
            census.banks,
            census.bank_streams,
            census.bank_samples,
            census.streamed_files,
            census.streamed_channels,
            census.streamed_samples,
            census.configs,
            census.config_names,
            rates.join(",\n"),
            channels.join(",\n"),
            census.failures.len(),
            census.known_vorbis,
            census.known_unparsed_configs.len()
        );
        fs::write(&out, json).unwrap();
    }

    let missed: Vec<_> = ANCHORS
        .iter()
        .enumerate()
        .filter(|(i, _)| !census.anchor_hits[*i])
        .map(|(_, a)| format!("{} {:#x} #{}", a.archive, a.hash, a.index))
        .collect();
    println!(
        "anchors_hit={}/{}",
        census.anchor_hits.iter().filter(|h| **h).count(),
        ANCHORS.len()
    );
    assert!(missed.is_empty(), "anchors never hit: {missed:?}");
    assert_eq!(
        census.known_vorbis, 1,
        "expected exactly the one Vorbis stream"
    );
    assert_eq!(
        census.known_unparsed_configs.len(),
        3,
        "expected exactly the 3 speech lookups"
    );
    assert!(
        census.failures.is_empty(),
        "{} failures (see output above)",
        census.failures.len()
    );
}
