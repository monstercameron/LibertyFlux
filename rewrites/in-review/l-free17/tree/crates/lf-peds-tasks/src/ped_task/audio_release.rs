//! The audio task's three object slots and the release that frees each.
//!
//! The task record holds three pointers to audio objects: a primary slot,
//! a radio slot and a secondary slot. Releasing a slot reads its object; a
//! null slot does nothing. For a nonnull object, the release first cleans
//! up the object's resource when the cleanup switch is not set, the game's
//! current mode is the sentinel and the skip mode is not the skip value, and
//! then releases the object. The three releases are one routine, so
//! [`AudioTask::release_slot`] is one generic method and the slot is an
//! argument.
//!
//! The game globals the release reads are passed in as
//! [`ReleaseGlobals`]. The object's resource word and the cleanup and
//! release helpers travel as [`AudioObjects`].

/// The cleanup switch value that disables cleanup.
pub const CLEANUP_ENABLED: u32 = 1;

/// The current-mode value that allows cleanup.
pub const CLEANUP_MODE_SENTINEL: u32 = 0xFFFF_FFFF;

/// The skip-mode value that suppresses cleanup.
pub const SKIP_MODE: u32 = 0x12;

/// The three audio object slots of the task record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioSlot {
    /// The primary slot.
    Primary,
    /// The radio slot.
    Radio,
    /// The secondary slot.
    Secondary,
}

/// The audio task's record fields that the audio routines use, owned: the
/// three object pointers, the progress pair and the deadline words.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AudioTask {
    /// The primary object pointer.
    pub primary: u32,
    /// The radio object pointer.
    pub radio: u32,
    /// The secondary object pointer.
    pub secondary: u32,
    /// The first progress value, a single-precision float.
    pub first_progress: f32,
    /// The second progress value, a single-precision float.
    pub second_progress: f32,
    /// The first deadline word, stamped with the current time.
    pub first_deadline: u32,
    /// The second deadline word, stamped with the current time; the window
    /// test reads it first.
    pub second_deadline: u32,
    /// The window test's second word. Only the window test reads it.
    pub window_deadline: u32,
}

/// Which deadline word a stamp writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Deadline {
    /// The first deadline word.
    First,
    /// The second deadline word.
    Second,
}

/// Length of the unexpired window, in ticks.
pub const WINDOW_TICKS: u32 = 1000;

/// Progress scale applied to the input before clamping.
pub const PROGRESS_SCALE: f32 = f32::from_bits(0x3DCC_CCCD);

/// Upper bound of both progress values.
pub const PROGRESS_LIMIT: f32 = f32::from_bits(0x3F80_0000);

/// Amount the second progress value advances by each call.
pub const PROGRESS_STEP: f32 = f32::from_bits(0x3ECC_CCCD);

/// The game globals a release reads, read by the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReleaseGlobals {
    /// The cleanup switch: release cleans up unless this equals
    /// [`CLEANUP_ENABLED`].
    pub cleanup_enabled: u32,
    /// The current mode: cleanup needs [`CLEANUP_MODE_SENTINEL`].
    pub current_mode: u32,
    /// The skip mode: cleanup is suppressed when it equals [`SKIP_MODE`].
    pub skip_mode: u32,
}

/// The reads and helper calls of a release. An object is an opaque handle.
pub trait AudioObjects {
    /// The resource word of an audio object.
    fn resource(&self, object: u32) -> u32;
    /// The cleanup helper, called with the object's resource.
    fn cleanup(&mut self, resource: u32);
    /// The release helper, called with the object.
    fn release(&mut self, object: u32);
}

impl AudioTask {
    /// The object held in `slot`.
    #[must_use]
    pub fn get(&self, slot: AudioSlot) -> u32 {
        match slot {
            AudioSlot::Primary => self.primary,
            AudioSlot::Radio => self.radio,
            AudioSlot::Secondary => self.secondary,
        }
    }

    /// Releases the object held in `slot`. A null slot does nothing. Otherwise
    /// the object's resource is cleaned up when the globals allow it, and the
    /// object is released last. The slot itself is not changed.
    pub fn release_slot<O: AudioObjects>(
        &self,
        slot: AudioSlot,
        globals: ReleaseGlobals,
        objects: &mut O,
    ) {
        let object = self.get(slot);
        if object == 0 {
            return;
        }
        if globals.cleanup_enabled != CLEANUP_ENABLED
            && globals.current_mode == CLEANUP_MODE_SENTINEL
            && globals.skip_mode != SKIP_MODE
        {
            let resource = objects.resource(object);
            objects.cleanup(resource);
        }
        objects.release(object);
    }
}
