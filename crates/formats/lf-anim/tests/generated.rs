//! Property and fuzz tests on generated animation dictionaries. Every byte
//! is generated here (see `crates/formats/tests/support.rs`); no game files
//! are needed.
//!
//! - Property: random dictionaries (base or episode tags, several clips
//!   with names hashed by [`lf_anim::clip_hash`], tracks whose channels are
//!   static quaternions, packed animated samples or unknown codecs, and an
//!   optional track pool) parse back to the same clips, hashes, names,
//!   frame counts, track ids and channel payloads; the hash and duration
//!   checks pass and lookups by name and hash find every clip.
//! - Fuzz: mutated system segments, re-wrapped in a valid container, never
//!   panic.

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

use lf_anim::{AnimDictionary, ChannelData, Codec, clip_hash};
use support::{Buf, Rng, fuzz, rsc5};

const SYS: u32 = 0x5000_0000;
const KIND_GENERIC: u32 = 1;
const ROOT_TAGS: [u32; 2] = [0x0069_5374, 0x0103_d0e0];
const CLIP_TAGS: [u32; 2] = [0x006a_ef64, 0x0107_8dbc];
const TAG_STATIC: [u32; 2] = [0x006a_f86c, 0x0107_b4e4];
const TAG_ANIMATED: [u32; 2] = [0x006a_f92c, 0x0107_b1dc];
const PARAM_STATIC: u32 = 0x900;
const PARAM_ANIMATED: u32 = 0x100;
/// Frame rate the clip duration is measured against.
const FPS: f32 = 30.0;

#[derive(Debug, Clone)]
enum ChanSpec {
    Static([f32; 4]),
    Animated(Vec<u32>),
    Opaque(u32, u32),
}

#[derive(Debug, Clone)]
struct ClipSpec {
    name: String,
    frames: u16,
    tracks: Vec<(u32, ChanSpec)>,
}

fn random_channel(rng: &mut Rng) -> ChanSpec {
    match rng.below(3) {
        0 => {
            // A unit quaternion from a normalised random vector.
            let v = [
                rng.f32_in(-1.0, 1.0),
                rng.f32_in(-1.0, 1.0),
                rng.f32_in(-1.0, 1.0),
                rng.f32_in(0.1, 1.0),
            ];
            let n = (v.iter().map(|x| x * x).sum::<f32>()).sqrt();
            ChanSpec::Static(v.map(|x| x / n))
        }
        1 => ChanSpec::Animated((0..rng.below(20)).map(|_| rng.next_u32()).collect()),
        // Unknown codec: a tag that is neither static nor animated, and a
        // data word that is not a system pointer.
        _ => ChanSpec::Opaque(
            0x0100_0000 | rng.below(0xFFFF) as u32,
            rng.next_u32() & 0x0FFF_FFFF,
        ),
    }
}

fn random_clips(rng: &mut Rng) -> Vec<ClipSpec> {
    (0..rng.range(1, 5))
        .map(|i| ClipSpec {
            name: format!("{}{i}", rng.ident(1, 12).to_lowercase()),
            frames: rng.range(1, 200) as u16,
            tracks: (0..rng.below(5))
                .map(|_| (rng.below(0x1000) as u32, random_channel(rng)))
                .collect(),
        })
        .collect()
}

struct Writer(Buf);

impl Writer {
    fn alloc(&mut self, len: usize) -> usize {
        self.0.align(16);
        let at = self.0.len();
        self.0.put(at, &vec![0; len.max(1)]);
        at
    }
    fn ptr(at: usize) -> u32 {
        SYS | at as u32
    }
    fn channel(&mut self, c: &ChanSpec, ep: usize) -> usize {
        let at = self.alloc(16);
        match c {
            ChanSpec::Static(q) => {
                let data = self.alloc(16);
                for (k, x) in q.iter().enumerate() {
                    self.0.put_f32(data + 4 * k, *x);
                }
                self.0
                    .put_u32(at, TAG_STATIC[ep])
                    .put_u32(at + 4, PARAM_STATIC)
                    .put_u32(at + 8, Writer::ptr(data));
            }
            ChanSpec::Animated(words) => {
                let data = self.alloc(words.len() * 4);
                for (k, w) in words.iter().enumerate() {
                    self.0.put_u32(data + 4 * k, *w);
                }
                let n = words.len() as u32;
                self.0
                    .put_u32(at, TAG_ANIMATED[ep])
                    .put_u32(at + 4, PARAM_ANIMATED)
                    .put_u32(at + 8, Writer::ptr(data))
                    .put_u32(at + 12, n | (n << 16));
            }
            ChanSpec::Opaque(tag, word2) => {
                self.0
                    .put_u32(at, *tag)
                    .put_u32(at + 8, *word2)
                    .put_u32(at + 12, 7);
            }
        }
        at
    }
}

/// Lay out a dictionary; `pool` adds a track pool with one entry per clip.
fn write_dict(clips: &[ClipSpec], ep: usize, pool: bool) -> Vec<u8> {
    let mut w = Writer(Buf::zeroed(0x30));
    let n = clips.len();
    let hashes = w.alloc(n * 4);
    let values = w.alloc(n * 4);
    for (i, c) in clips.iter().enumerate() {
        let name = w.alloc(c.name.len() + 12);
        w.0.put(name, format!("pack:/{}.anim", c.name).as_bytes());
        let mut descs = Vec::new();
        for (id, chan) in &c.tracks {
            let channel = w.channel(chan, ep);
            let track = w.alloc(0x18);
            w.0.put_u32(track, *id)
                .put_u32(track + 4, Writer::ptr(channel))
                .put_u32(track + 0x14, 1);
            let group = w.alloc(4);
            w.0.put_u32(group, Writer::ptr(track));
            let desc = w.alloc(16);
            w.0.put_u32(desc, *id)
                .put_u32(desc + 4, 0x0F)
                .put_u32(desc + 8, Writer::ptr(group))
                .put_u32(desc + 12, 0x0001_0001);
            descs.push(desc);
        }
        let table = w.alloc(descs.len() * 4);
        for (k, d) in descs.iter().enumerate() {
            w.0.put_u32(table + 4 * k, Writer::ptr(*d));
        }
        let t = c.tracks.len() as u32;
        let clip = w.alloc(0x30);
        w.0.put_u32(clip, CLIP_TAGS[ep])
            .put_u32(clip + 4, 0x01f1_0001)
            .put_u16(clip + 8, c.frames)
            .put_u16(clip + 10, 15)
            .put_f32(clip + 12, f32::from(c.frames - 1) / FPS)
            .put_u32(clip + 16, rand_id(i))
            .put_u32(clip + 0x1C, Writer::ptr(name))
            .put_u32(clip + 0x20, Writer::ptr(table))
            .put_u32(clip + 0x24, t | (t << 16))
            .put_u32(clip + 0x28, 0x142);
        w.0.put_u32(hashes + 4 * i, clip_hash(&c.name))
            .put_u32(values + 4 * i, Writer::ptr(clip));
    }
    let pair = n as u32 | ((n as u32) << 16);
    w.0.put_u32(0, ROOT_TAGS[ep])
        .put_u32(4, Writer::ptr(0x30))
        .put_u32(12, 1)
        .put_u32(0x10, Writer::ptr(hashes))
        .put_u32(0x14, pair)
        .put_u32(0x18, Writer::ptr(values))
        .put_u32(0x1C, pair)
        .put_u32(0x28, 7)
        .put_u32(0x2C, 9);
    if pool {
        let entries: Vec<usize> = (0..n)
            .map(|i| {
                let ch = w.channel(&ChanSpec::Static([0.0, 0.0, 0.0, 1.0]), ep);
                let e = w.alloc(16);
                w.0.put_u32(e, 0x1000 + i as u32)
                    .put_u32(e + 4, Writer::ptr(ch))
                    .put_u32(e + 8, 0)
                    .put_u32(e + 12, 1);
                e
            })
            .collect();
        let arr = w.alloc(n * 4);
        for (k, e) in entries.iter().enumerate() {
            w.0.put_u32(arr + 4 * k, Writer::ptr(*e));
        }
        w.0.put_u32(0x20, Writer::ptr(arr)).put_u32(0x24, pair);
    }
    w.0.0
}

fn rand_id(i: usize) -> u32 {
    0x1234_0000 + i as u32
}

#[test]
fn dictionary_round_trip() {
    let mut rng = Rng::for_test("anim round trip");
    for case in 0..100 {
        let clips = random_clips(&mut rng);
        let ep = case % 2;
        let pool = case % 3 != 0;
        let file = rsc5(KIND_GENERIC, &write_dict(&clips, ep, pool), &[]);
        let dict = AnimDictionary::parse_bytes(&file).expect("generated dictionary parses");
        assert_eq!(dict.build_tag(), ROOT_TAGS[ep]);
        assert_eq!((dict.root_unknown_28(), dict.root_unknown_2c()), (7, 9));
        assert_eq!(dict.len(), clips.len());
        for (i, (clip, want)) in dict.clips().iter().zip(&clips).enumerate() {
            assert_eq!(clip.short_name(), want.name);
            assert_eq!(clip.name(), format!("pack:/{}.anim", want.name));
            assert_eq!(clip.hash(), clip_hash(&want.name));
            assert_eq!(clip.frames(), want.frames);
            assert_eq!(clip.clip_id(), rand_id(i));
            assert!(clip.validate_hash());
            assert!(clip.validate_frame_duration());
            assert_eq!(
                dict.clip_by_name(&want.name).map(lf_anim::Clip::hash),
                Some(clip.hash())
            );
            assert!(dict.clip_by_hash(clip.hash()).is_some());
            assert_eq!(clip.tracks().len(), want.tracks.len());
            let mut statics = 0;
            for (track, (id, chan)) in clip.tracks().iter().zip(&want.tracks) {
                assert_eq!(track.id(), *id);
                let ch = track.channel();
                match (chan, ch.data()) {
                    (ChanSpec::Static(q), ChannelData::StaticQuat(g)) => {
                        assert_eq!(ch.codec(), Codec::StaticQuaternion);
                        assert_eq!([g.x, g.y, g.z, g.w], *q);
                        assert!(g.is_unit(1e-4));
                        statics += 1;
                    }
                    (ChanSpec::Animated(words), ChannelData::Samples { words: got }) => {
                        assert_eq!(ch.codec(), Codec::AnimatedPacked);
                        assert_eq!(got, words);
                    }
                    (ChanSpec::Opaque(tag, word2), ChannelData::Opaque { word2: g2, word3 }) => {
                        assert_eq!(ch.codec(), Codec::Unknown);
                        assert_eq!((ch.tag(), *g2, *word3), (*tag, *word2, 7));
                    }
                    other => panic!("channel mismatch: {other:?}"),
                }
            }
            assert_eq!(clip.static_pose().len(), statics);
        }
        match dict.pool() {
            Some(p) => {
                assert!(pool);
                assert_eq!(p.len(), clips.len());
                for (i, e) in p.iter().enumerate() {
                    assert_eq!(e.id(), 0x1000 + i as u32);
                    assert_eq!(e.channels().len(), 1);
                    assert_eq!(e.trailer(), [0, 1]);
                }
            }
            None => assert!(!pool),
        }
    }
}

#[test]
fn fuzz_system_segments() {
    let mut rng = Rng::for_test("anim fuzz");
    let seeds: Vec<Vec<u8>> = (0..4)
        .map(|i| write_dict(&random_clips(&mut rng), i % 2, i != 0))
        .collect();
    fuzz("anim system segment", &seeds, 3000, |sys| {
        if let Ok(dict) = AnimDictionary::parse_bytes(&rsc5(KIND_GENERIC, sys, &[])) {
            for clip in dict.clips() {
                let _ = (
                    clip.validate_hash(),
                    clip.validate_frame_duration(),
                    clip.static_pose(),
                );
                let _ = dict.clip_by_name(clip.short_name());
            }
            if let Some(pool) = dict.pool() {
                let _ = pool.iter().map(|p| p.channels().len()).sum::<usize>();
            }
        }
    });
}
