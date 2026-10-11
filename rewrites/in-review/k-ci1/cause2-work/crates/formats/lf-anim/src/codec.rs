//! Channel codecs: static and animated track data.
//!
//! Every track's animation data lives in a channel object. The object opens
//! with a build tag identifying the codec class, a parameter word, and a data
//! pointer; what follows depends on the codec. Base-game and episode tags
//! differ (different asset builds) but share parameter words and layouts.
//!
//! Decoded so far:
//!
//! * [`Codec::StaticQuaternion`]: four plain little-endian floats, a unit
//!   quaternion used for every frame of the track.
//! * [`Codec::AnimatedPacked`]: one packed u32 per frame. The packing is
//!   unknown; the words are exposed raw in frame order.
//!
//! Anything else is [`ChannelData::Opaque`]: the header words are recorded
//! and no payload is claimed.

use lf_resource::{Pointer, Resource, Segment};

use crate::{Error, Result};

/// Build tags of the eight channel codecs shipped in animation files.
/// Each codec has a base-game tag and an episode-asset twin (same parameter
/// word and layout, different stale build address).
///
/// | codec (inferred role) | base tag | episode tag | param |
/// |---|---|---|---|
/// | packed animated samples | 0x6AF92C | 0x107B1DC | 0x100 |
/// | static scalar, inline value (?) | 0x6AF77C | 0x107B3FC | 0x400 |
/// | small-count samples (?) | 0x6AFDDC | 0x107BBFC | 0x600 |
/// | rare counted codec (?) | 0x6AF98C | 0x107B2CC | 0x700 |
/// | static quaternion | 0x6AF86C | 0x107B4E4 | 0x900 |
/// | static zero/bool (?) | 0x6AF7DC | 0x107B57C | 0xB00 |
/// | counted codec (?) | 0x6AFE7C | 0x107BD74 | 0xC00 |
/// | static (?) | 0x6AF8CC | 0x107B60C | 0xD00 |
pub const TAG_ANIMATED_BASE: u32 = 0x6a_f92c;
/// Episode twin of [`TAG_ANIMATED_BASE`].
pub const TAG_ANIMATED_EP: u32 = 0x107_b1dc;
/// Base-game tag of the 0x400 codec (role unknown).
pub const TAG_400_BASE: u32 = 0x6a_f77c;
/// Episode twin of [`TAG_400_BASE`].
pub const TAG_400_EP: u32 = 0x107_b3fc;
/// Base-game tag of the 0x600 codec (role unknown; the most common codec).
pub const TAG_600_BASE: u32 = 0x6a_fddc;
/// Episode twin of [`TAG_600_BASE`].
pub const TAG_600_EP: u32 = 0x107_bbfc;
/// Base-game tag of the rare 0x700 codec (role unknown).
pub const TAG_700_BASE: u32 = 0x6a_f98c;
/// Episode twin of [`TAG_700_BASE`].
pub const TAG_700_EP: u32 = 0x107_b2cc;
/// Base-game tag of the static-quaternion codec.
pub const TAG_STATIC_QUAT_BASE: u32 = 0x6a_f86c;
/// Episode twin of [`TAG_STATIC_QUAT_BASE`].
pub const TAG_STATIC_QUAT_EP: u32 = 0x107_b4e4;
/// Base-game tag of the 0xB00 codec (role unknown).
pub const TAG_B00_BASE: u32 = 0x6a_f7dc;
/// Episode twin of [`TAG_B00_BASE`].
pub const TAG_B00_EP: u32 = 0x107_b57c;
/// Base-game tag of the 0xC00 codec (role unknown).
pub const TAG_C00_BASE: u32 = 0x6a_fe7c;
/// Episode twin of [`TAG_C00_BASE`].
pub const TAG_C00_EP: u32 = 0x107_bd74;
/// Base-game tag of the 0xD00 codec (role unknown).
pub const TAG_D00_BASE: u32 = 0x6a_f8cc;
/// Episode twin of [`TAG_D00_BASE`].
pub const TAG_D00_EP: u32 = 0x107_b60c;

/// Parameter word always seen with [`Codec::StaticQuaternion`].
pub const PARAM_STATIC_QUAT: u32 = 0x900;
/// Parameter word always seen with [`Codec::AnimatedPacked`].
pub const PARAM_ANIMATED: u32 = 0x100;

/// A rotation as four floats in x, y, z, w order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
    /// W component.
    pub w: f32,
}

impl Quaternion {
    /// Euclidean norm.
    #[must_use]
    pub fn norm(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    /// True when the norm is 1 within `tolerance`.
    #[must_use]
    pub fn is_unit(self, tolerance: f32) -> bool {
        (self.norm() - 1.0).abs() <= tolerance
    }
}

/// The codec a channel object uses, identified by its build tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Codec {
    /// Four plain floats: a unit quaternion constant over the clip.
    StaticQuaternion,
    /// One packed u32 per frame; the packing is not yet understood.
    AnimatedPacked,
    /// A tag this crate does not decode.
    Unknown,
}

impl Codec {
    /// Classify a raw build tag.
    #[must_use]
    pub const fn from_tag(tag: u32) -> Codec {
        if tag == TAG_STATIC_QUAT_BASE || tag == TAG_STATIC_QUAT_EP {
            Codec::StaticQuaternion
        } else if tag == TAG_ANIMATED_BASE || tag == TAG_ANIMATED_EP {
            Codec::AnimatedPacked
        } else {
            Codec::Unknown
        }
    }
}

/// A channel's payload.
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelData {
    /// A rotation constant over the whole clip.
    StaticQuat(Quaternion),
    /// Packed per-frame samples in frame order; one word per frame.
    /// The packing is unknown (see the crate documentation).
    Samples {
        /// Sample words, one per frame.
        words: Vec<u32>,
    },
    /// An undecoded channel: header words two and three are recorded raw
    /// and no payload is read. (Header length differs per codec: some
    /// channels are 12 bytes with an inline value, so neither word is
    /// assumed to be a pointer.)
    Opaque {
        /// Raw third header word (word at offset 8).
        word2: u32,
        /// Raw fourth header word (word at offset 12).
        word3: u32,
    },
}

/// One animation channel: a codec tag, its parameter word, and its payload.
#[derive(Debug, Clone, PartialEq)]
pub struct Channel {
    tag: u32,
    codec: Codec,
    param: u32,
    data: ChannelData,
}

impl Channel {
    /// Parse the channel object at system offset `offset`.
    ///
    /// Reads the four header words with bounds checks, then decodes the
    /// payload for known codecs. Unknown tags yield [`ChannelData::Opaque`]
    /// rather than an error, so new codecs never break parsing.
    ///
    /// # Errors
    ///
    /// Returns an error when the header words or the payload run past the
    /// system segment or the data pointer leaves it.
    pub fn parse(res: &Resource, offset: usize) -> Result<Channel> {
        let sys = res.system();
        let tag = read_u32(sys, offset)?;
        let param = read_u32(sys, offset + 4)?;
        let data_raw = read_u32(sys, offset + 8)?;
        let word3 = read_u32(sys, offset + 12)?;
        let codec = Codec::from_tag(tag);
        let data = match codec {
            Codec::StaticQuaternion => {
                let ptr = Pointer::new(data_raw);
                require_system(ptr, offset + 8)?;
                let bytes = res.slice(ptr, 16)?;
                ChannelData::StaticQuat(Quaternion {
                    x: f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
                    y: f32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
                    z: f32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
                    w: f32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
                })
            }
            Codec::AnimatedPacked => {
                let ptr = Pointer::new(data_raw);
                require_system(ptr, offset + 8)?;
                let frames = (word3 & 0xFFFF) as usize;
                let byte_len = frames.checked_mul(4).ok_or(Error::BadCount {
                    offset: offset + 12,
                    found: word3,
                })?;
                let bytes = res.slice(ptr, byte_len)?;
                let mut samples = Vec::with_capacity(frames);
                for chunk in bytes.chunks_exact(4) {
                    samples.push(u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
                }
                ChannelData::Samples { words: samples }
            }
            Codec::Unknown => ChannelData::Opaque {
                word2: data_raw,
                word3,
            },
        };
        Ok(Channel {
            tag,
            codec,
            param,
            data,
        })
    }

    /// The raw build tag word.
    #[must_use]
    pub fn tag(&self) -> u32 {
        self.tag
    }

    /// The classified codec.
    #[must_use]
    pub fn codec(&self) -> Codec {
        self.codec
    }

    /// The parameter word (0x900 for static quaternions, 0x100 for packed
    /// animated channels in every shipped file).
    #[must_use]
    pub fn param(&self) -> u32 {
        self.param
    }

    /// The decoded (or opaque) payload.
    #[must_use]
    pub fn data(&self) -> &ChannelData {
        &self.data
    }
}

/// Read a little-endian u32 from the system segment with a bounds check.
fn read_u32(sys: &[u8], offset: usize) -> Result<u32> {
    let bytes =
        sys.get(offset..offset + 4)
            .ok_or(Error::Resource(lf_resource::Error::OutOfBounds {
                value: 0x5000_0000 | (u32::try_from(offset).unwrap_or(u32::MAX) & 0x0FFF_FFFF),
                len: 4,
                segment_len: sys.len(),
            }))?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

/// Reject a channel data pointer that is not a system-segment pointer.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quaternions_measure_unit() {
        let q = Quaternion {
            x: 0.0,
            y: 0.0,
            z: -0.066,
            w: 0.998,
        };
        assert!(q.is_unit(1e-2));
        assert!(!q.is_unit(1e-6));
        let flat = Quaternion {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };
        assert!(!flat.is_unit(1e-2));
    }

    #[test]
    fn codec_tags_classify() {
        assert_eq!(
            Codec::from_tag(TAG_STATIC_QUAT_BASE),
            Codec::StaticQuaternion
        );
        assert_eq!(Codec::from_tag(TAG_ANIMATED_BASE), Codec::AnimatedPacked);
        assert_eq!(Codec::from_tag(0x1234_5678), Codec::Unknown);
    }
}
