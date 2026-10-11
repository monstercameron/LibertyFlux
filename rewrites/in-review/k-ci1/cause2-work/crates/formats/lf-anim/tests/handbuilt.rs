//! Tests on small hand-built dictionaries.
//!
//! Every byte below is constructed in this file; nothing comes from game
//! data. A minimal system segment is laid out by hand, wrapped in an RSC5
//! container with zlib compression, and parsed through the public API.

use std::io::Write;

use flate2::Compression;
use flate2::write::ZlibEncoder;

use lf_anim::{AnimDictionary, ChannelData, Codec};

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
    let flags: u32 = u32::try_from(sys.len() / 256).expect("fixture fits in u32");
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
fn one_clip(channel_tag: u32, channel_param: u32) -> Vec<u8> {
    let mut sys = vec![0u8; 512];
    w32(&mut sys, 0x00, 0x69_5374); // root tag
    w32(&mut sys, 0x04, 0x5000_0300); // block area (content unchecked)
    w32(&mut sys, 0x0C, 1);
    w32(&mut sys, 0x10, 0x5000_0040); // hashes
    w32(&mut sys, 0x14, 0x0001_0001);
    w32(&mut sys, 0x18, 0x5000_0050); // values
    w32(&mut sys, 0x1C, 0x0001_0001);
    // no pool: 0x20 stays null
    w32(&mut sys, 0x40, lf_anim::clip_hash("hop"));
    w32(&mut sys, 0x50, 0x5000_0060); // clip
    w32(&mut sys, 0x60, 0x6a_ef64); // clip tag
    w32(&mut sys, 0x64, 0x01f1_0001); // flags
    w16(&mut sys, 0x68, 11); // frames
    w16(&mut sys, 0x6A, 15); // unknown u16
    wf32(&mut sys, 0x6C, 10.0 / 30.0); // duration
    w32(&mut sys, 0x70, 0x1234_5678); // clip id
    w32(&mut sys, 0x74, 0x5000_0200); // group (unchecked)
    w32(&mut sys, 0x78, 0x0001_0001);
    w32(&mut sys, 0x7C, 0x5000_0100); // name
    w32(&mut sys, 0x80, 0x5000_0090); // track table
    w32(&mut sys, 0x84, 0x0001_0001);
    w32(&mut sys, 0x88, 0x142);
    w32(&mut sys, 0x90, 0x5000_00A0); // desc
    w32(&mut sys, 0xA0, 0x380); // track id
    w32(&mut sys, 0xA4, 0x0F); // flags
    w32(&mut sys, 0xA8, 0x5000_00B0); // group
    w32(&mut sys, 0xAC, 0x0001_0001);
    w32(&mut sys, 0xB0, 0x5000_00C0); // track
    w32(&mut sys, 0xC0, 0x380); // id echo
    w32(&mut sys, 0xC4, 0x5000_00E0); // channel
    w32(&mut sys, 0xD4, 1);
    w32(&mut sys, 0xE0, channel_tag);
    w32(&mut sys, 0xE4, channel_param);
    w32(&mut sys, 0xE8, 0x5000_00F0); // data
    w32(&mut sys, 0xEC, 0);
    // static quat (0,0,0,1) doubles as animated-sample words for other tags
    wf32(&mut sys, 0xF0, 0.0);
    wf32(&mut sys, 0xF4, 0.0);
    wf32(&mut sys, 0xF8, 0.0);
    wf32(&mut sys, 0xFC, 1.0);
    sys[0x100..0x100 + 15].copy_from_slice(b"pack:/hop.anim\0");
    rsc(&sys)
}

#[test]
fn parses_hand_built_static_clip() {
    let file = one_clip(0x6a_f86c, 0x900);
    let dict = AnimDictionary::parse_bytes(&file).unwrap();
    assert_eq!(dict.len(), 1);
    assert!(dict.pool().is_none());
    let clip = &dict.clips()[0];
    assert_eq!(clip.short_name(), "hop");
    assert_eq!(clip.name(), "pack:/hop.anim");
    assert_eq!(clip.frames(), 11);
    assert_eq!(clip.track_u16(), 15);
    assert!(clip.validate_frame_duration());
    assert!(clip.validate_hash());
    assert_eq!(clip.tracks().len(), 1);
    let track = &clip.tracks()[0];
    assert_eq!(track.id(), 0x380);
    assert_eq!(track.channel().codec(), Codec::StaticQuaternion);
    match track.channel().data() {
        ChannelData::StaticQuat(q) => {
            assert_eq!((q.x, q.y, q.z, q.w), (0.0, 0.0, 0.0, 1.0));
            assert!(q.is_unit(1e-6));
        }
        other => panic!("want static quat, got {other:?}"),
    }
    let pose = clip.static_pose();
    assert_eq!(pose.len(), 1);
    assert_eq!(pose[0].0, 0x380);
    assert_eq!(dict.clip_by_name("hop").unwrap().hash(), clip.hash());
    assert!(dict.clip_by_hash(0xdead_beef).is_none());
}

#[test]
fn unknown_codec_stays_opaque() {
    let file = one_clip(0x1111_2222, 0x3333_4444);
    let dict = AnimDictionary::parse_bytes(&file).unwrap();
    let track = &dict.clips()[0].tracks()[0];
    assert_eq!(track.channel().codec(), Codec::Unknown);
    match track.channel().data() {
        ChannelData::Opaque { word2, word3 } => {
            assert_eq!(*word2, 0x5000_00F0);
            assert_eq!(*word3, 0);
        }
        other => panic!("want opaque, got {other:?}"),
    }
    // No static tracks: empty pose, but parsing still succeeds.
    assert!(dict.clips()[0].static_pose().is_empty());
}

#[test]
fn animated_samples_read_in_frame_order() {
    // Same one-clip layout as above, animated tag, count 3, then 3 words.
    let mut sys = vec![0u8; 512];
    w32(&mut sys, 0x00, 0x69_5374);
    w32(&mut sys, 0x0C, 1);
    w32(&mut sys, 0x10, 0x5000_0040);
    w32(&mut sys, 0x14, 0x0001_0001);
    w32(&mut sys, 0x18, 0x5000_0050);
    w32(&mut sys, 0x1C, 0x0001_0001);
    w32(&mut sys, 0x40, lf_anim::clip_hash("run"));
    w32(&mut sys, 0x50, 0x5000_0060);
    w32(&mut sys, 0x60, 0x6a_ef64);
    w32(&mut sys, 0x64, 0x01f1_0001);
    w16(&mut sys, 0x68, 3);
    w16(&mut sys, 0x6A, 15);
    wf32(&mut sys, 0x6C, 2.0 / 30.0);
    w32(&mut sys, 0x7C, 0x5000_0100);
    w32(&mut sys, 0x80, 0x5000_0090);
    w32(&mut sys, 0x84, 0x0001_0001);
    w32(&mut sys, 0x90, 0x5000_00A0);
    w32(&mut sys, 0xA0, 0x381);
    w32(&mut sys, 0xA4, 0x0F);
    w32(&mut sys, 0xA8, 0x5000_00B0);
    w32(&mut sys, 0xAC, 0x0001_0001);
    w32(&mut sys, 0xB0, 0x5000_00C0);
    w32(&mut sys, 0xC0, 0x381);
    w32(&mut sys, 0xC4, 0x5000_00E0);
    w32(&mut sys, 0xE0, 0x6a_f92c);
    w32(&mut sys, 0xE4, 0x100);
    w32(&mut sys, 0xE8, 0x5000_00F0);
    w32(&mut sys, 0xEC, 0x0003_0003); // 3 samples
    w32(&mut sys, 0xF0, 0xAAAA_AAAA);
    w32(&mut sys, 0xF4, 0xBBBB_BBBB);
    w32(&mut sys, 0xF8, 0xCCCC_CCCC);
    sys[0x100..0x100 + 15].copy_from_slice(b"pack:/run.anim\0");
    let dict = AnimDictionary::parse_bytes(&rsc(&sys)).unwrap();
    let track = &dict.clips()[0].tracks()[0];
    assert_eq!(track.channel().codec(), Codec::AnimatedPacked);
    match track.channel().data() {
        ChannelData::Samples { words } => {
            assert_eq!(words, &vec![0xAAAA_AAAA, 0xBBBB_BBBB, 0xCCCC_CCCC]);
        }
        other => panic!("want samples, got {other:?}"),
    }
}

#[test]
fn parses_pool_entries() {
    // One clip plus a one-entry pool with a static channel.
    let mut sys = vec![0u8; 1024];
    w32(&mut sys, 0x00, 0x69_5374);
    w32(&mut sys, 0x0C, 1);
    w32(&mut sys, 0x10, 0x5000_0040);
    w32(&mut sys, 0x14, 0x0001_0001);
    w32(&mut sys, 0x18, 0x5000_0050);
    w32(&mut sys, 0x1C, 0x0001_0001);
    w32(&mut sys, 0x20, 0x5000_0200); // pool array
    w32(&mut sys, 0x24, 0x0001_0001);
    w32(&mut sys, 0x28, 7);
    w32(&mut sys, 0x2C, 9);
    w32(&mut sys, 0x40, lf_anim::clip_hash("hop"));
    w32(&mut sys, 0x50, 0x5000_0060);
    w32(&mut sys, 0x60, 0x6a_ef64);
    w32(&mut sys, 0x64, 0x01f1_0001);
    w16(&mut sys, 0x68, 11);
    w16(&mut sys, 0x6A, 15);
    wf32(&mut sys, 0x6C, 10.0 / 30.0);
    w32(&mut sys, 0x7C, 0x5000_0100);
    w32(&mut sys, 0x80, 0x5000_0090);
    w32(&mut sys, 0x84, 0x0001_0001);
    w32(&mut sys, 0x90, 0x5000_00A0);
    w32(&mut sys, 0xA0, 0x380);
    w32(&mut sys, 0xA4, 0x0F);
    w32(&mut sys, 0xA8, 0x5000_00B0);
    w32(&mut sys, 0xAC, 0x0001_0001);
    w32(&mut sys, 0xB0, 0x5000_00C0);
    w32(&mut sys, 0xC0, 0x380);
    w32(&mut sys, 0xC4, 0x5000_00E0);
    w32(&mut sys, 0xE0, 0x6a_f86c);
    w32(&mut sys, 0xE4, 0x900);
    w32(&mut sys, 0xE8, 0x5000_00F0);
    wf32(&mut sys, 0xFC, 1.0);
    sys[0x100..0x100 + 15].copy_from_slice(b"pack:/hop.anim\0");
    // pool: array -> entry -> one static channel, then trailer words
    w32(&mut sys, 0x200, 0x5000_0210);
    w32(&mut sys, 0x210, 0x1000); // pool id
    w32(&mut sys, 0x214, 0x5000_0220); // channel
    w32(&mut sys, 0x218, 0); // trailer
    w32(&mut sys, 0x21C, 1); // trailer
    w32(&mut sys, 0x220, 0x6a_f86c);
    w32(&mut sys, 0x224, 0x900);
    w32(&mut sys, 0x228, 0x5000_0230);
    wf32(&mut sys, 0x23C, 1.0);
    let dict = AnimDictionary::parse_bytes(&rsc(&sys)).unwrap();
    assert_eq!(dict.root_unknown_28(), 7);
    assert_eq!(dict.root_unknown_2c(), 9);
    let pool = dict.pool().unwrap();
    assert_eq!(pool.len(), 1);
    assert_eq!(pool[0].id(), 0x1000);
    assert_eq!(pool[0].trailer(), [0, 1]);
    assert_eq!(pool[0].channels().len(), 1);
    assert!(matches!(
        pool[0].channels()[0].data(),
        ChannelData::StaticQuat(_)
    ));
}

#[test]
fn rejects_garbage() {
    assert!(AnimDictionary::parse_bytes(&[]).is_err());
    assert!(AnimDictionary::parse_bytes(&[0u8; 64]).is_err());
    // Right magic, wrong kind.
    let mut sys = vec![0u8; 256];
    w32(&mut sys, 0x00, 0x69_5374);
    let mut file = rsc(&sys);
    file[4..8].copy_from_slice(&8u32.to_le_bytes()); // texture kind
    let err = AnimDictionary::parse_bytes(&file).unwrap_err();
    assert!(matches!(err, lf_anim::Error::UnexpectedKind { found: 8 }));
    // Right kind, wrong root tag.
    let mut sys = vec![0u8; 256];
    w32(&mut sys, 0x00, 0x1234_5678);
    let err = AnimDictionary::parse_bytes(&rsc(&sys)).unwrap_err();
    assert!(matches!(err, lf_anim::Error::BadHeader { offset: 0, .. }));
    // Truncated inside the clip table (pointer off the end).
    let mut sys = vec![0u8; 256];
    w32(&mut sys, 0x00, 0x69_5374);
    w32(&mut sys, 0x0C, 1);
    w32(&mut sys, 0x10, 0x5000_00F0);
    w32(&mut sys, 0x14, 0x0001_0001);
    w32(&mut sys, 0x18, 0x5000_00F4);
    w32(&mut sys, 0x1C, 0x0001_0001);
    w32(&mut sys, 0xF0, 0x1234_5678);
    w32(&mut sys, 0xF4, 0x5000_FFF0); // off the end
    assert!(AnimDictionary::parse_bytes(&rsc(&sys)).is_err());
}
