//! The game camera object and its three lifted methods.
//!
//! Field names say what each word is used for where that is established by
//! the verified rewrites; words whose use is not established keep their
//! offset in the name and are marked in the field docs. Bit masks name the
//! marker each method sets or clears.

use lf_core::Handle32;

use super::world::{CamGameWorld, CamOwnerTag, CamTag};

/// Mode word an idle camera holds (set by the idle reset).
pub const IDLE_MODE: u32 = 3;

/// Timer word an idle camera holds (set by the idle reset).
pub const IDLE_TIMER: u32 = 0x7530;

/// Marker bit of the first flag byte that the idle reset sets.
pub const IDLE_MARKER: u8 = 0x80;

/// Marker bit of the first flag byte that the restart clears.
pub const ACTIVE_MARKER: u8 = 0x40;

/// The zoom an idle camera holds, as a float.
pub const DEFAULT_ZOOM: f32 = 1.0;

/// Worker mode the restart asks for.
const WORKER_MODE: u32 = 1;

/// Extra word the restart passes when it asks for a worker.
const WORKER_EXTRA: u32 = 0;

/// Low flag bit of the deleting destructor: return the storage to the pool.
const RELEASE_TO_POOL: u32 = 1;

/// The camera state the three lifted methods read or write.
///
/// The comments give each field's offset within the original object, which
/// is the only place an address appears; the lifted methods never see one.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CCamGame {
    /// The owner the camera is attached to (offset 0x114). `None` for zero.
    pub owner: Option<Handle32<CamOwnerTag>>,
    /// Mode word (offset 0x140). The idle reset sets it to [`IDLE_MODE`].
    pub mode: u32,
    /// Byte beside the mode word (offset 0x144). Cleared by the idle reset.
    pub mode_flag: u8,
    /// Default word copied from the shared global (offset 0x148).
    pub default_word: u32,
    /// Timer word (offset 0x14C). The idle reset sets it to [`IDLE_TIMER`].
    pub timer: u32,
    /// Byte beside the timer word (offset 0x150). Cleared by the idle reset.
    pub timer_flag: u8,
    /// Word cleared by the idle reset (offset 0x154). Use not established.
    pub word_154: u32,
    /// Zoom, as the bits of a float (offset 0x1BC). The idle reset sets 1.0.
    pub zoom_bits: u32,
    /// First flag byte (offset 0x1C4). The idle reset clears its low three
    /// bits before the reset step and bits 3 to 5 after it, then sets
    /// [`IDLE_MARKER`]; the restart clears [`ACTIVE_MARKER`].
    pub flags_1c4: u8,
    /// Second flag byte (offset 0x1C5). The idle reset clears bits 1 to 3
    /// before the reset step and bit 0 after it.
    pub flags_1c5: u8,
    /// Pending word (offset 0x1C8), cleared by the restart. Use not established.
    pub pending_1c8: u32,
    /// Pending word (offset 0x1CC), cleared by the restart. Use not established.
    pub pending_1cc: u32,
    /// Pending word (offset 0x1D0), cleared by the restart. Use not established.
    pub pending_1d0: u32,
    /// Word cleared by the idle reset (offset 0x1D4). Use not established.
    pub word_1d4: u32,
    /// Byte cleared by the idle reset (offset 0x1D8). Use not established.
    pub flag_1d8: u8,
    /// Word cleared by the idle reset (offset 0x1DC). Use not established.
    pub word_1dc: u32,
    /// Word cleared by the idle reset (offset 0x1E0). Use not established.
    pub word_1e0: u32,
    /// Word cleared by the idle reset (offset 0x1E4). Use not established.
    pub word_1e4: u32,
    /// Word cleared by the idle reset (offset 0x1E8). Use not established.
    pub word_1e8: u32,
}

impl CCamGame {
    /// Deleting destructor.
    ///
    /// Runs the class's destructor body first; then, when the low bit of
    /// `flags` is set, asks the pool manager to take the camera's storage
    /// back. The original's return value (the object's own identity) is not
    /// modelled: the owner already holds `this`.
    pub fn deleting_destructor(
        &mut self,
        world: &mut impl CamGameWorld,
        this: Handle32<CamTag>,
        flags: u32,
    ) {
        world.destroy_body(self, this);
        if flags & RELEASE_TO_POOL != 0 {
            let manager = world.pool_manager();
            world.pool_release(manager, this);
        }
    }

    /// Restart.
    ///
    /// Runs the camera's own teardown step, builds a worker for the owner,
    /// chains the worker's flags, releases the worker, then clears the
    /// active marker and the pending words. The original always answers 1;
    /// that constant is not returned.
    pub fn restart(&mut self, world: &mut impl CamGameWorld, this: Handle32<CamTag>) {
        world.teardown(self, this);
        let owner = self.owner;
        let worker = world.create_worker(owner, WORKER_MODE, WORKER_EXTRA, this);
        world.chain_set_flags(worker);
        world.release_worker(worker);
        self.flags_1c4 &= !ACTIVE_MARKER;
        self.pending_1c8 = 0;
        self.pending_1cc = 0;
        self.pending_1d0 = 0;
    }

    /// Idle reset.
    ///
    /// Clears the mode, timer and flag words, copies the shared default word
    /// into its slot and runs the shared reset step. The reset step may
    /// change the flag byte, so the byte is read again after it: bits 3 to 5
    /// are cleared from whatever the step left, and the idle marker is set.
    /// Then it sets the default zoom and clears the remaining words. The
    /// original always answers 1; that constant is not returned.
    pub fn reset_to_idle(&mut self, world: &mut impl CamGameWorld, this: Handle32<CamTag>) {
        self.flags_1c5 &= 0xf1;
        self.word_1e4 = 0;
        self.word_1e8 = 0;
        self.word_1e0 = 0;
        self.mode_flag = 0;
        self.mode = IDLE_MODE;
        self.timer_flag = 0;
        self.flags_1c4 &= 0xf8;
        self.default_word = world.shared_default_word();
        self.timer = IDLE_TIMER;
        world.reset_state_block(self, this);
        let flags = self.flags_1c4;
        self.flags_1c5 &= 0xfe;
        self.flags_1c4 = (flags & 0xc7) | IDLE_MARKER;
        self.zoom_bits = DEFAULT_ZOOM.to_bits();
        self.flag_1d8 = 0;
        self.word_1d4 = 0;
        self.word_154 = 0;
        self.word_1dc = 0;
    }
}
