//! Read-only parser for the game's animation dictionaries and clips (`.wad`).
//!
//! A WAD file is one animation dictionary: a set of named clips that animate
//! one skeleton. Dictionaries live inside the animation archives (`anim.img`
//! holds one WAD per animation group) and inside map archives for animated
//! props. Every WAD is an RSC5 resource (see [`lf_resource`]) of generic type
//! with a system segment only: there is no graphics segment.
//!
//! This crate was written from the author's own inspection of the game's
//! files. It contains no game data. Public documentation contributed only the
//! dictionary's name ("Windows Animation Dictionary") and the hash function
//! family; every layout claim below was verified against real files (see the
//! integration test) unless marked unknown.
//!
//! # Format description
//!
//! All integers are little-endian. Offsets below are byte offsets into the
//! decompressed system segment. Internal references are tagged resource
//! pointers ([`lf_resource::Pointer`]); words that look like raw executable
//! addresses are build-time type tags, not pointers (see below).
//!
//! ## Dictionary root (system offset 0)
//!
//! ```text
//! offset  size  content
//! 0x00    4     build tag: 0x695374 in the base game, 0x103D0E0 in the two
//!               episodes (same layout, different asset build)
//! 0x04    4     system pointer to a small block holding zero plus debug fill
//! 0x08    4     zero
//! 0x0C    4     one
//! 0x10    4     system pointer to the hash array (u32 per clip)
//! 0x14    4     hash count as two u16 (count, capacity; always equal)
//! 0x18    4     system pointer to the value array (system pointer per clip)
//! 0x1C    4     value count as two u16 (always equal to the hash count)
//! 0x20    4     system pointer to the file track pool, or null when absent
//!               (absent in one shipped file only)
//! 0x24    4     pool count as two u16 (count, capacity; always equal)
//! 0x28    4     unknown u32
//! 0x2C    4     unknown u32 (often, but not always, a clip frame count)
//! ```
//!
//! Each hash is the Jenkins one-at-a-time hash ([`hash::clip_hash`]) of the
//! clip's short name: the file stem of its `pack:/<name>.anim` path, matched
//! on every one of the tens of thousands of shipped clips.
//!
//! ## Clip (dictionary value)
//!
//! ```text
//! offset  size  content
//! 0x00    4     build tag: 0x6AEF64 base, 0x1078DBC episodes
//! 0x04    4     flags; only four distinct values ship (see [`Clip::flags`])
//! 0x08    2     frame count (samples per channel, 30 Hz)
//! 0x0A    2     unknown u16; always one less than a multiple of 16
//! 0x0C    4     float duration in seconds, always (frames - 1) / 30
//! 0x10    4     unknown u32 clip id (usually differs per clip)
//! 0x14    4     system pointer to the runtime track group
//! 0x18    4     group count as two u16 (one in every shipped file)
//! 0x1C    4     system pointer to the NUL-terminated name ("pack:/<name>.anim")
//! 0x20    4     system pointer to the track table (system pointer per track)
//! 0x24    4     track count as two u16 (always equal)
//! 0x28    4     unknown u32
//! 0x2C    4     zero
//! ```
//!
//! ## Track descriptor (clip table entry target)
//!
//! ```text
//! offset  size  content
//! 0x00    4     track id (bone or morph target; id space is per skeleton)
//! 0x04    4     flags; observed equal to the clip's unknown u16 each time
//! 0x08    4     system pointer to a one-entry group holding the track
//! 0x0C    4     group count as two u16 (one in every shipped file)
//! ```
//!
//! ## Track and channel
//!
//! A track holds one animated node: a track id echo, a pointer to its channel
//! object, and four unknown words. A channel object starts with a build tag
//! identifying the codec, a parameter word, and a data pointer; the rest is
//! codec specific (see [`codec`]):
//!
//! * Static rotation channels store four plain floats (x, y, z, w): a unit
//!   quaternion, verified unit length on all 273,089 shipped instances.
//! * One animated codec stores one packed u32 per frame; the packing is
//!   **unknown** and the words are exposed raw for a later lane.
//! * Six further codecs exist (see [`codec`] for the tag table); their
//!   headers are recorded but their payloads are exposed raw.
//!
//! ## Build tags
//!
//! The tag words (`0x69…`, `0x6A…` in the base game, `0x103…`/`0x107…` in the
//! episodes) match no vtable in the shipped executable: they are stale
//! absolute addresses from the asset build and serve only to tell classes
//! apart. Base and episode tags with the same parameter words and layout are
//! treated as the same codec (see [`codec::Codec`]).
//!
//! ## File track pool (root offset 0x20)
//!
//! The pool holds one entry per track with inline channel objects in the same
//! tag/parameter/data shape as clip channels. Its exact role is unknown: the
//! channel sample counts match the first clip's frame count, so it may mirror
//! the first clip or pre-build shared runtime state. It is parsed
//! defensively (ids and channel headers only) and never trusted for sample
//! extents.
//!
//! ## What is still unknown
//!
//! * The packing of animated per-frame sample words (one u32 per frame).
//! * The meaning of the clip u16 at 0x0A, the clip id at 0x10, the words at
//!   0x28/0x2C, the root words at 0x28/0x2C, and the track id namespace.
//! * The exact role of the file track pool and of the runtime track group.
//! * Payload layouts of the rarer codec tags (recorded, not decoded).
//!
//! # Example
//!
//! ```no_run
//! use lf_anim::AnimDictionary;
//!
//! let bytes = std::fs::read("amb@arcade.wad").unwrap();
//! let dict = AnimDictionary::parse_bytes(&bytes)?;
//! for clip in dict.clips() {
//!     println!("{}: {} frames, {:.3}s", clip.short_name(), clip.frames(), clip.duration());
//! }
//! # Ok::<(), lf_anim::Error>(())
//! ```

mod codec;
mod dictionary;
mod error;
mod hash;

pub use codec::{
    Channel, ChannelData, Codec, PARAM_ANIMATED, PARAM_STATIC_QUAT, Quaternion, TAG_400_BASE,
    TAG_400_EP, TAG_600_BASE, TAG_600_EP, TAG_700_BASE, TAG_700_EP, TAG_ANIMATED_BASE,
    TAG_ANIMATED_EP, TAG_B00_BASE, TAG_B00_EP, TAG_C00_BASE, TAG_C00_EP, TAG_D00_BASE, TAG_D00_EP,
    TAG_STATIC_QUAT_BASE, TAG_STATIC_QUAT_EP,
};
pub use dictionary::{AnimDictionary, Clip, PoolTrack, Track};
pub use error::Error;
pub use hash::clip_hash;

/// Shorthand for `Result<T, lf_anim::Error>`.
pub type Result<T> = std::result::Result<T, Error>;
