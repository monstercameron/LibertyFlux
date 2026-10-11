//! The voice tracker: a held voice, its count and its linked record.
//!
//! Lifted from the verified rewrites of the four `0x9E` tracker routines
//! (no class): install a voice/count pair, release the held voice and park
//! the linked record, install-and-bind, resolve-and-attach. The 32-bit
//! object carries the voice word at `+0x30`, the linked-record address at
//! `+0x34` and the count at `+0x38`; the linked record carries a tag byte
//! at `+0x40` and a cached word at `+0x48`. The lift owns the three words
//! and the two record fields; the voice itself and the pool stay opaque
//! cookies, and every numbered callee becomes one [`TrackerWorld`] method.

use lf_core::Handle32;

/// Identity of a voice object owned elsewhere.
#[derive(Debug)]
pub struct VoiceTag;

/// An opaque voice: the tracker's held voice word.
pub type VoiceHandle = Handle32<VoiceTag>;

/// Identity of the voice pool behind the install routine's global.
#[derive(Debug)]
pub struct PoolTag;

/// An opaque voice pool.
pub type PoolHandle = Handle32<PoolTag>;

/// Tag byte the release parks onto the linked record.
pub const PARK_TAG: u8 = 0xFF;
/// Cached word the release parks onto the linked record.
pub const PARK_CACHED: u32 = 0xFFFF_FFFF;
/// Handle value meaning "no voice resolved".
pub const INVALID_HANDLE: u32 = 0xFFFF_FFFF;

/// The linked record's two modelled fields (record `+0x40`/`+0x48`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinkedSlot {
    /// The tag byte.
    pub tag: u8,
    /// The cached word.
    pub cached: u32,
}

/// A voice tracker: held voice, linked record, count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VoiceTracker {
    /// The held voice (`+0x30`; `None` is the null word).
    pub voice: Option<VoiceHandle>,
    /// The linked record (`+0x34`; `None` is the null word).
    pub link: Option<LinkedSlot>,
    /// The count (`+0x38`).
    pub count: u32,
}

impl VoiceTracker {
    /// A tracker from its three words.
    #[must_use]
    pub const fn new(voice: Option<VoiceHandle>, link: Option<LinkedSlot>, count: u32) -> Self {
        Self { voice, link, count }
    }
}

/// The tracker's collaborators: every numbered callee of the four
/// rewrites, plus the pool behind the install routine's global.
///
/// The 32-bit callees also receive the tracker's own address; it carries
/// no meaning in the lift (the tracker is `self`) and is dropped, with
/// the proof pinning the planted address per case.
pub trait TrackerWorld {
    /// Releases a held voice: the release routine's helper.
    fn release(&mut self, voice: VoiceHandle);
    /// Installs a voice/count pair into a tracker: the install-and-bind
    /// routine's first callee. The effect on the tracker is the caller's
    /// to script (the proof runs both install-like and empty scripts).
    fn install_pair(&mut self, tracker: &mut VoiceTracker, voice: VoiceHandle, count: u32);
    /// Binds a tracker to a voice's owner: the install-and-bind
    /// routine's second callee, answering the bind result.
    fn bind(&mut self, owner: Option<VoiceHandle>) -> u32;
    /// Resolves the voice handle for an argument: the resolve routine's
    /// first callee, answering the handle or [`INVALID_HANDLE`].
    fn resolve(&mut self, voice: VoiceHandle, arg: u32) -> u32;
    /// Attaches a resolved handle: the resolve routine's second callee.
    fn attach(&mut self, handle: u32);
    /// Refreshes a linked record's cached value through the pool: the
    /// install routine's callee, answering the fresh value.
    fn refresh(&mut self, pool: Option<PoolHandle>, voice: Option<VoiceHandle>) -> u32;
}

impl VoiceTracker {
    /// Installs a voice/count pair and refreshes the linked record.
    ///
    /// Stores `voice` and `count`; without a linked record answers
    /// `count`. Otherwise stamps the count's low byte onto the record's
    /// tag, refreshes its cached word through `pool`, and answers the
    /// fresh value.
    pub fn install(
        &mut self,
        voice: Option<VoiceHandle>,
        count: u32,
        pool: Option<PoolHandle>,
        world: &mut impl TrackerWorld,
    ) -> u32 {
        self.voice = voice;
        self.count = count;
        let Some(slot) = self.link.as_mut() else {
            return count;
        };
        // The original stamps the count's low byte: truncation intended.
        #[allow(clippy::cast_possible_truncation)]
        {
            slot.tag = count as u8;
        }
        let fresh = world.refresh(pool, voice);
        slot.cached = fresh;
        fresh
    }

    /// Releases the held voice and parks the linked record.
    ///
    /// When a voice is held and the count is positive (signed), the
    /// voice is released and the count cleared; the voice word is
    /// cleared either way. A linked record is parked to
    /// ([`PARK_TAG`], [`PARK_CACHED`]). Answers whether a record was
    /// linked (the original answers its address; the proof reconstructs
    /// it per case).
    pub fn release_and_park(&mut self, world: &mut impl TrackerWorld) -> bool {
        if let Some(voice) = self.voice {
            if self.count.cast_signed() > 0 {
                world.release(voice);
                self.count = 0;
            }
            self.voice = None;
        }
        let Some(slot) = self.link.as_mut() else {
            return false;
        };
        slot.tag = PARK_TAG;
        slot.cached = PARK_CACHED;
        true
    }

    /// Installs a voice/count pair, then binds to the voice's owner.
    ///
    /// Without a voice answers 0 with no calls. Otherwise the pair is
    /// installed through the world (which sets the tracker's words),
    /// the resulting voice word is read back as the owner, and the bind
    /// answer is returned.
    pub fn install_and_bind(
        &mut self,
        voice: Option<VoiceHandle>,
        count: u32,
        world: &mut impl TrackerWorld,
    ) -> u32 {
        let Some(voice) = voice else { return 0 };
        world.install_pair(self, voice, count);
        let owner = self.voice;
        world.bind(owner)
    }

    /// Resolves the voice handle for an argument, then attaches it.
    ///
    /// Answers false with no calls when no voice is held, or when the
    /// resolved handle is [`INVALID_HANDLE`]; otherwise attaches the
    /// handle and answers true.
    pub fn handle_resolve(&mut self, arg: u32, world: &mut impl TrackerWorld) -> bool {
        let Some(voice) = self.voice else {
            return false;
        };
        let handle = world.resolve(voice, arg);
        if handle == INVALID_HANDLE {
            return false;
        }
        world.attach(handle);
        true
    }
}
