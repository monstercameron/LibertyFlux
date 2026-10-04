//! The animation dictionary: clips, tracks, and the file track pool.
//!
//! See the [crate documentation](crate) for the byte layout.

use lf_resource::{Pointer, Resource, ResourceKind, Segment};

use crate::codec::{Channel, ChannelData, Quaternion};
use crate::{Error, Result, hash};

/// Build tag at the root of base-game dictionaries.
pub const ROOT_TAG_BASE: u32 = 0x69_5374;
/// Build tag at the root of episode dictionaries (same layout).
pub const ROOT_TAG_EPISODE: u32 = 0x103_d0e0;
/// Build tag at the head of base-game clips.
pub const CLIP_TAG_BASE: u32 = 0x6a_ef64;
/// Build tag at the head of episode clips (same layout).
pub const CLIP_TAG_EPISODE: u32 = 0x107_8dbc;

/// Samples per second of every animated channel.
pub const SAMPLE_RATE: f32 = 30.0;

/// Largest name this parser will read (shipped names are far shorter).
const MAX_NAME: usize = 256;

/// One animation dictionary: the clips of one `.wad` file.
#[derive(Debug, Clone)]
pub struct AnimDictionary {
    build_tag: u32,
    clips: Vec<Clip>,
    pool: Option<Vec<PoolTrack>>,
    root_unknown_28: u32,
    root_unknown_2c: u32,
}

impl AnimDictionary {
    /// Parse a whole `.wad` file from memory.
    ///
    /// The bytes must be one RSC5 resource of the generic kind with a
    /// system segment only.
    ///
    /// # Errors
    ///
    /// Returns an error when the bytes are not a system-only generic
    /// resource or the animation structures inside fail to parse.
    pub fn parse_bytes(bytes: &[u8]) -> Result<AnimDictionary> {
        let res = Resource::parse(bytes)?;
        AnimDictionary::parse_resource(&res)
    }

    /// Parse an already-decompressed [`Resource`].
    ///
    /// # Errors
    ///
    /// Returns an error when the resource is not a system-only generic
    /// resource or the animation structures inside fail to parse.
    pub fn parse_resource(res: &Resource) -> Result<AnimDictionary> {
        if res.header().kind != ResourceKind::GENERIC {
            return Err(Error::UnexpectedKind {
                found: res.header().kind.raw(),
            });
        }
        if !res.graphics().is_empty() {
            return Err(Error::UnexpectedGraphics {
                len: res.graphics().len(),
            });
        }
        let sys = res.system();
        let build_tag = read_u32(sys, 0)?;
        if build_tag != ROOT_TAG_BASE && build_tag != ROOT_TAG_EPISODE {
            return Err(Error::BadHeader {
                offset: 0,
                expected: "animation root tag",
                found: build_tag,
            });
        }
        let w08 = read_u32(sys, 8)?;
        if w08 != 0 {
            return Err(Error::BadHeader {
                offset: 8,
                expected: "zero",
                found: w08,
            });
        }
        let w0c = read_u32(sys, 12)?;
        if w0c != 1 {
            return Err(Error::BadHeader {
                offset: 12,
                expected: "one",
                found: w0c,
            });
        }
        let hash_ptr = sys_pointer(sys, 0x10)?;
        let hash_count = count_word(sys, 0x14)?;
        let value_ptr = sys_pointer(sys, 0x18)?;
        let value_count = count_word(sys, 0x1C)?;
        if hash_count != value_count {
            return Err(Error::BadCount {
                offset: 0x1C,
                found: value_count,
            });
        }
        sanity_count(hash_count, sys.len(), 8)?;
        let mut clips = Vec::with_capacity(hash_count.min(4096) as usize);
        for i in 0..hash_count as usize {
            let hash = res.read_u32(advance(hash_ptr, i * 4)?)?;
            let clip_at = res.read_pointer(advance(value_ptr, i * 4)?)?;
            clips.push(Clip::parse(res, hash, clip_at)?);
        }
        // Optional file track pool (null in one shipped file).
        let pool_raw = read_u32(sys, 0x20)?;
        let pool = if pool_raw == 0 {
            None
        } else {
            let pool_ptr = Pointer::new(pool_raw);
            require_system(pool_ptr, 0x20)?;
            let pool_count = count_word(sys, 0x24)?;
            sanity_count(pool_count, sys.len(), 4)?;
            let mut tracks = Vec::with_capacity(pool_count.min(4096) as usize);
            for i in 0..pool_count as usize {
                let at = res.read_pointer(advance(pool_ptr, i * 4)?)?;
                tracks.push(PoolTrack::parse(res, at)?);
            }
            Some(tracks)
        };
        Ok(AnimDictionary {
            build_tag,
            clips,
            pool,
            root_unknown_28: read_u32(sys, 0x28)?,
            root_unknown_2c: read_u32(sys, 0x2C)?,
        })
    }

    /// The root build tag (tells base-game and episode assets apart).
    #[must_use]
    pub fn build_tag(&self) -> u32 {
        self.build_tag
    }

    /// The clips in dictionary order.
    #[must_use]
    pub fn clips(&self) -> &[Clip] {
        &self.clips
    }

    /// Number of clips.
    #[must_use]
    pub fn len(&self) -> usize {
        self.clips.len()
    }

    /// True when the dictionary holds no clips.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.clips.is_empty()
    }

    /// Find a clip by its dictionary hash (see [`hash::clip_hash`]).
    #[must_use]
    pub fn clip_by_hash(&self, hash: u32) -> Option<&Clip> {
        self.clips.iter().find(|c| c.hash == hash)
    }

    /// Find a clip by short name (hashes the name first).
    #[must_use]
    pub fn clip_by_name(&self, short_name: &str) -> Option<&Clip> {
        self.clip_by_hash(hash::clip_hash(short_name))
    }

    /// The file track pool, when present. Its exact role is unknown; see
    /// the [crate documentation](crate).
    #[must_use]
    pub fn pool(&self) -> Option<&[PoolTrack]> {
        self.pool.as_deref()
    }

    /// Unknown root word at 0x28.
    #[must_use]
    pub fn root_unknown_28(&self) -> u32 {
        self.root_unknown_28
    }

    /// Unknown root word at 0x2C.
    #[must_use]
    pub fn root_unknown_2c(&self) -> u32 {
        self.root_unknown_2c
    }
}

/// One animation clip: a name, a length, and a track per animated node.
#[derive(Debug, Clone)]
pub struct Clip {
    hash: u32,
    name: String,
    short_name: String,
    flags: u32,
    frames: u16,
    track_u16: u16,
    duration: f32,
    id: u32,
    unknown_28: u32,
    tracks: Vec<Track>,
}

impl Clip {
    /// Parse the clip at `at` with dictionary hash `hash`.
    fn parse(res: &Resource, hash: u32, at: Pointer) -> Result<Clip> {
        require_system(at, 0)?;
        let base = at.offset();
        let sys = res.system();
        let tag = read_u32(sys, base)?;
        if tag != CLIP_TAG_BASE && tag != CLIP_TAG_EPISODE {
            return Err(Error::BadHeader {
                offset: base,
                expected: "clip tag",
                found: tag,
            });
        }
        let flags = read_u32(sys, base + 4)?;
        let frames = read_u16(sys, base + 8)?;
        let track_u16 = read_u16(sys, base + 10)?;
        let duration = read_f32(sys, base + 12)?;
        let id = read_u32(sys, base + 16)?;
        let name_ptr = Pointer::new(read_u32(sys, base + 0x1C)?);
        require_system(name_ptr, base + 0x1C)?;
        let name = res
            .read_cstring(name_ptr, MAX_NAME)
            .map_err(|_| Error::BadName {
                offset: name_ptr.offset(),
            })?
            .to_owned();
        let short_name = short_stem(&name);
        let table_ptr = Pointer::new(read_u32(sys, base + 0x20)?);
        require_system(table_ptr, base + 0x20)?;
        let track_count = count_word(sys, base + 0x24)?;
        sanity_count(track_count, sys.len(), 4)?;
        let unknown_28 = read_u32(sys, base + 0x28)?;
        let w2c = read_u32(sys, base + 0x2C)?;
        if w2c != 0 {
            return Err(Error::BadHeader {
                offset: base + 0x2C,
                expected: "zero",
                found: w2c,
            });
        }
        let mut tracks = Vec::with_capacity(track_count.min(4096) as usize);
        for i in 0..track_count as usize {
            let desc_at = res.read_pointer(advance(table_ptr, i * 4)?)?;
            tracks.push(Track::parse(res, desc_at)?);
        }
        Ok(Clip {
            hash,
            name,
            short_name,
            flags,
            frames,
            track_u16,
            duration,
            id,
            unknown_28,
            tracks,
        })
    }

    /// The dictionary hash ([`hash::clip_hash`] of the short name).
    #[must_use]
    pub fn hash(&self) -> u32 {
        self.hash
    }

    /// The full stored name (`pack:/<name>.anim`).
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The file stem of the name (the hashed part).
    #[must_use]
    pub fn short_name(&self) -> &str {
        &self.short_name
    }

    /// Flags word; only four distinct values ship.
    #[must_use]
    pub fn flags(&self) -> u32 {
        self.flags
    }

    /// Samples per channel.
    #[must_use]
    pub fn frames(&self) -> u16 {
        self.frames
    }

    /// Unknown u16 beside the frame count (always 16k - 1).
    #[must_use]
    pub fn track_u16(&self) -> u16 {
        self.track_u16
    }

    /// Length in seconds: always `(frames - 1) / 30`.
    #[must_use]
    pub fn duration(&self) -> f32 {
        self.duration
    }

    /// Unknown per-clip id.
    #[must_use]
    pub fn clip_id(&self) -> u32 {
        self.id
    }

    /// Unknown word at clip offset 0x28.
    #[must_use]
    pub fn unknown_28(&self) -> u32 {
        self.unknown_28
    }

    /// The clip's tracks in table order.
    #[must_use]
    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    /// True when `duration` equals `(frames - 1) / 30` within tolerance.
    /// Holds for every shipped clip.
    #[must_use]
    pub fn validate_frame_duration(&self) -> bool {
        if self.frames == 0 {
            return false;
        }
        let expect = f32::from(self.frames - 1) / SAMPLE_RATE;
        (self.duration - expect).abs() <= 1e-4
    }

    /// True when the dictionary hash matches the short name.
    /// Holds for every shipped clip.
    #[must_use]
    pub fn validate_hash(&self) -> bool {
        hash::clip_hash(&self.short_name) == self.hash
    }

    /// The clip's constant rotations: one (track id, quaternion) per track
    /// whose channel is a static quaternion.
    ///
    /// Tracks with animated or undecoded channels are skipped; use
    /// [`Clip::tracks`] to reach their raw samples.
    #[must_use]
    pub fn static_pose(&self) -> Vec<(u32, Quaternion)> {
        self.tracks
            .iter()
            .filter_map(|t| match t.channel.data() {
                ChannelData::StaticQuat(q) => Some((t.id, *q)),
                _ => None,
            })
            .collect()
    }
}

/// One track: a node id and its channel.
#[derive(Debug, Clone)]
pub struct Track {
    id: u32,
    flags: u32,
    channel: Channel,
}

impl Track {
    /// Parse the track descriptor at `desc_at`.
    fn parse(res: &Resource, desc_at: Pointer) -> Result<Track> {
        require_system(desc_at, 0)?;
        let base = desc_at.offset();
        let sys = res.system();
        let id = read_u32(sys, base)?;
        let flags = read_u32(sys, base + 4)?;
        let group_ptr = Pointer::new(read_u32(sys, base + 8)?);
        require_system(group_ptr, base + 8)?;
        // Group count is one in every shipped file; the first entry is used.
        let track_at = res.read_pointer(group_ptr)?;
        require_system(track_at, group_ptr.offset())?;
        let sys = res.system();
        let channel_at = Pointer::new(read_u32(sys, track_at.offset() + 4)?);
        require_system(channel_at, track_at.offset() + 4)?;
        let channel = Channel::parse(res, channel_at.offset())?;
        Ok(Track { id, flags, channel })
    }

    /// Track id (bone or morph target; namespace is per skeleton).
    #[must_use]
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Descriptor flags (always equal to the clip's unknown u16;
    /// verified on every shipped track).
    #[must_use]
    pub fn flags(&self) -> u32 {
        self.flags
    }

    /// The track's channel.
    #[must_use]
    pub fn channel(&self) -> &Channel {
        &self.channel
    }
}

/// One file-pool track: an id and inline channel headers.
///
/// The pool's role is unknown; entries are parsed defensively and channel
/// payloads are decoded only for codecs whose layout is verified.
#[derive(Debug, Clone)]
pub struct PoolTrack {
    id: u32,
    channels: Vec<Channel>,
    trailer: [u32; 2],
}

impl PoolTrack {
    /// Parse the pool entry at `at`: an id, consecutive channel pointers,
    /// then two raw trailer words.
    fn parse(res: &Resource, at: Pointer) -> Result<PoolTrack> {
        require_system(at, 0)?;
        let sys = res.system();
        let mut offset = at.offset();
        let id = read_u32(sys, offset)?;
        offset += 4;
        let mut channels = Vec::new();
        for _ in 0..64 {
            let raw = read_u32(sys, offset)?;
            let ptr = Pointer::new(raw);
            if ptr.segment() != Some(Segment::System) {
                break;
            }
            // A channel header is at least 16 bytes; stop at the edge.
            if ptr.offset() + 16 > sys.len() {
                break;
            }
            channels.push(Channel::parse(res, ptr.offset())?);
            offset += 4;
        }
        let trailer = [read_u32(sys, offset)?, read_u32(sys, offset + 4)?];
        Ok(PoolTrack {
            id,
            channels,
            trailer,
        })
    }

    /// Pool entry id.
    #[must_use]
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Inline channels in entry order.
    #[must_use]
    pub fn channels(&self) -> &[Channel] {
        &self.channels
    }

    /// The two raw words after the channel pointers.
    #[must_use]
    pub fn trailer(&self) -> [u32; 2] {
        self.trailer
    }
}

/// The file stem of `pack:/<name>.anim` (the hashed part).
fn short_stem(name: &str) -> String {
    let after_slash = name.rsplit('/').next().unwrap_or(name);
    match after_slash.rfind('.') {
        Some(dot) => after_slash[..dot].to_owned(),
        None => after_slash.to_owned(),
    }
}

/// Read a little-endian u16 with a bounds check.
fn read_u16(sys: &[u8], offset: usize) -> Result<u16> {
    let bytes = sys
        .get(offset..offset + 2)
        .ok_or(out_of_bounds(offset, 2, sys.len()))?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

/// Read a little-endian u32 with a bounds check.
fn read_u32(sys: &[u8], offset: usize) -> Result<u32> {
    let bytes = sys
        .get(offset..offset + 4)
        .ok_or(out_of_bounds(offset, 4, sys.len()))?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

/// Read a little-endian f32 with a bounds check.
fn read_f32(sys: &[u8], offset: usize) -> Result<f32> {
    Ok(f32::from_le_bytes(read_u32(sys, offset)?.to_le_bytes()))
}

/// Read a system pointer from an absolute system offset.
fn sys_pointer(sys: &[u8], offset: usize) -> Result<Pointer> {
    let ptr = Pointer::new(read_u32(sys, offset)?);
    require_system(ptr, offset)?;
    Ok(ptr)
}

/// Read a (count, capacity) u16 pair; the low half is the count.
fn count_word(sys: &[u8], offset: usize) -> Result<u32> {
    Ok(u32::from(read_u16(sys, offset)?))
}

/// Advance a pointer by `bytes`, checked.
fn advance(ptr: Pointer, bytes: usize) -> Result<Pointer> {
    let offset = ptr.offset().checked_add(bytes).ok_or(Error::BadCount {
        offset: 0,
        found: u32::try_from(bytes).unwrap_or(u32::MAX),
    })?;
    if offset > Pointer::OFFSET_MASK as usize {
        return Err(Error::BadCount {
            offset: 0,
            found: u32::try_from(offset).unwrap_or(u32::MAX),
        });
    }
    let tag = match ptr.segment() {
        Some(Segment::System) => 5u32,
        Some(Segment::Graphics) => 6u32,
        None => {
            return Err(Error::BadCount {
                offset: 0,
                found: ptr.raw(),
            });
        }
    };
    // The bound check above keeps `offset` inside the 28-bit mask, so this
    // conversion cannot fail; the fallback only satisfies the types.
    Ok(Pointer::new(
        tag << 28 | u32::try_from(offset).unwrap_or(u32::MAX),
    ))
}

/// Reject absurd counts before looping.
fn sanity_count(count: u32, seg_len: usize, entry_bytes: usize) -> Result<()> {
    let max = (seg_len / entry_bytes.max(1)) as u64;
    if u64::from(count) > max {
        return Err(Error::BadCount {
            offset: 0,
            found: count,
        });
    }
    Ok(())
}

/// Reject a pointer that is not a system-segment pointer.
fn require_system(ptr: Pointer, offset: usize) -> Result<()> {
    if ptr.segment() != Some(Segment::System) {
        return Err(Error::BadHeader {
            offset,
            expected: "system pointer",
            found: ptr.raw(),
        });
    }
    Ok(())
}

fn out_of_bounds(offset: usize, len: usize, segment_len: usize) -> Error {
    Error::Resource(lf_resource::Error::OutOfBounds {
        value: 0x5000_0000 | (u32::try_from(offset).unwrap_or(u32::MAX) & 0x0FFF_FFFF),
        len,
        segment_len,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_stem_shapes() {
        assert_eq!(short_stem("pack:/yawn.anim"), "yawn");
        assert_eq!(short_stem("play_pinball.anim"), "play_pinball");
        assert_eq!(short_stem("noname"), "noname");
    }
}
