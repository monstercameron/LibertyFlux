//! Lifted audio voices: turning sample data into device sound.
//!
//! The original plays every sound through a voice object. All voices
//! share one header (a flag byte, a sample-rate word, a parameter block
//! with the level word) and one twelve-slot virtual interface; each
//! subclass renders differently:
//!
//! - [`SoftVoice`](soft::SoftVoice): the software mixer voice, rendered
//!   by the engine mixer through a child voice.
//! - [`PcAdpcmVoice`](pc_adpcm::PcAdpcmVoice): ADPCM synthesis through
//!   the mixer, with codec and predictor tables.
//! - [`DSoundVoice`](dsound::DSoundVoice): a voice played through a
//!   sound device object.
//! - [`AdpcmVoice`](dsound_adpcm::AdpcmVoice): ADPCM synthesis onto a
//!   sound device.
//!
//! Each type owns its header words and its sample lanes as ordinary Rust
//! data (no addresses, no virtual tables, no allocator calls) and each
//! verified 32-bit method with behaviour in it is restated as a method.
//! Collaborator objects (child voices, devices, codecs, the mixer, the
//! parameter block) are carried as opaque [`Handle32`] cookies and
//! reached through one trait per class (`SoftWorld`, `PcWorld`,
//! `DSoundWorld`, `AdpcmWorld`): every callee slot and every virtual
//! call of the verified rewrites is one trait method, so dispatch is
//! static and tests script answers through a fake.
//!
//! The twelve virtual slots, as established from the verified rewrites
//! (Verified per class; the cross-class reading is Inferred):
//!
//! | Slot | Meaning |
//! |---|---|
//! | 0 | deleting destructor through the voice pool (Drop covers it) |
//! | 1 | attach to a playback request (mixer, buffer, effect chain) |
//! | 2 | teardown: release child/devices/buffer, run the base entry |
//! | 3 | start or seek onto the child or channel, report the level |
//! | 4 | resume on the child or device, unless not pending |
//! | 5 | stop the child or device |
//! | 6 | stopping-status query over flags, child and parameters |
//! | 7 | lane-remainder-zero query over the ring cursor |
//! | 8 | refresh the current ring lane or queue a synth block |
//! | 10 | current playback position, or all-ones when not playing |
//! | 11 | refresh child mode, relay the level, restart |
//!
//! Slot 9 is not observed in this family's inventory. Proof is
//! differential: every lifted method runs against its verified rewrite
//! on the same generated inputs, comparing results, every effect, and
//! every collaborator call in order, floats bit for bit (see the
//! `lf-voicediff` test crate). Nothing here is verified by the checker
//! itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].
//!
//! [`Handle32`]: lf_core::Handle32

#![forbid(unsafe_code)]

pub mod dsound;
pub mod dsound_adpcm;
pub mod pc_adpcm;
pub mod registry;
pub mod shared;
pub mod soft;

pub use dsound::{DSoundLane, DSoundVoice, DSoundWorld};
pub use dsound_adpcm::{AdpcmLane, AdpcmVoice, AdpcmWorld};
pub use pc_adpcm::{PcAdpcmVoice, PcLane, PcWorld};
pub use soft::{SoftLane, SoftVoice, SoftWorld};

/// Tag for the opaque cookie of a child voice object.
pub struct ChildTag;
/// Tag for the opaque cookie of a sound device or channel object.
pub struct DeviceTag;
/// Tag for the opaque cookie of an ADPCM codec object.
pub struct CodecTag;
/// Tag for the opaque cookie of a mixing buffer handle.
pub struct BufferTag;
/// Tag for the opaque cookie of an auxiliary engine object.
pub struct AuxTag;
/// Tag for the opaque cookie of the mixer singleton.
pub struct MixerTag;
