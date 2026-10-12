//! The audio task record: its three object slots and the release that frees
//! each, its deadline words and the unexpired-window test, and its progress pair.
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
    /// The subtask pointer: the object the subtask slot holds.
    pub subtask: u32,
    /// The subtask value word.
    pub subtask_value: u32,
    /// The subtask state word.
    pub subtask_state: u32,
    /// The subtask status word.
    pub subtask_status: u32,
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

    /// Stamps the chosen deadline word with the current game time.
    pub fn stamp_deadline(&mut self, deadline: Deadline, now: u32) {
        match deadline {
            Deadline::First => self.first_deadline = now,
            Deadline::Second => self.second_deadline = now,
        }
    }

    /// Whether the current time is still inside the window that either of the
    /// second and window deadline words opens: each opens [`WINDOW_TICKS`]
    /// ticks after its stamp, and the comparison is unsigned with wrapping
    /// addition.
    #[must_use]
    pub fn has_unexpired_window(&self, now: u32) -> bool {
        now < self.second_deadline.wrapping_add(WINDOW_TICKS)
            || now < self.window_deadline.wrapping_add(WINDOW_TICKS)
    }

    /// Advances the progress pair from one input. The input is scaled and
    /// capped at the limit; the first progress value only moves up to it and
    /// never down. The second progress value advances by the step and is
    /// capped at the limit. The float operations run in this order.
    pub fn add_float_progress(&mut self, input: f32) {
        let mut scaled = input * PROGRESS_SCALE;
        if scaled > PROGRESS_LIMIT {
            scaled = PROGRESS_LIMIT;
        }
        if !(scaled > self.first_progress) {
            scaled = self.first_progress;
        }
        self.first_progress = scaled;

        let advanced = self.second_progress + PROGRESS_STEP;
        self.second_progress = if advanced > PROGRESS_LIMIT {
            PROGRESS_LIMIT
        } else {
            advanced
        };
    }
}

/// Mask of the candidate's kind bits, read from its flags word.
pub const CANDIDATE_KIND_MASK: u32 = 0x3C0;

/// The kind bits a special candidate carries.
pub const CANDIDATE_KIND_SPECIAL: u32 = 0x80;

/// The candidate state that makes a special candidate's value apply.
pub const CANDIDATE_STATE_READY: u32 = 1;

/// The reads and helper calls of a subtask replacement. A candidate is an
/// opaque handle; its two words are read only when it is nonnull.
pub trait SubtaskCallees {
    /// The describe helper, called with the subtask object being replaced.
    /// The 32-bit helper also receives the address of the record's subtask
    /// slot; the implementation supplies it.
    fn describe(&mut self, previous: u32);
    /// The construct helper, called with the new candidate. The 32-bit
    /// helper also receives the address of the record's subtask slot; the
    /// implementation supplies it. It is called even for a null candidate.
    fn construct(&mut self, candidate: u32);
    /// The candidate's flags word.
    fn candidate_flags(&self, candidate: u32) -> u32;
    /// The candidate's state word.
    fn candidate_state(&self, candidate: u32) -> u32;
    /// The object the default-object global points at; its word is the
    /// value a null initial value stores.
    fn default_object(&self) -> u32;
    /// The special value global, which a ready special candidate stores.
    fn special_value(&self) -> u32;
}

impl AudioTask {
    /// Replaces the subtask. A nonnull previous subtask is described and then
    /// cleared. A null initial value stores the default object, clears the
    /// state and status, and stops. Otherwise the value is the initial
    /// value, unless a ready special candidate replaces it with the special
    /// value. The state is cleared, the candidate is stored in the slot and
    /// constructed, and the status is stored last.
    pub fn replace_subtask<C: SubtaskCallees>(
        &mut self,
        initial: u32,
        candidate: u32,
        final_status: u32,
        callees: &mut C,
    ) {
        let previous = self.subtask;
        if previous != 0 {
            callees.describe(previous);
            self.subtask = 0;
        }

        if initial == 0 {
            self.subtask_value = callees.default_object();
            self.subtask_state = 0;
            self.subtask_status = 0;
            return;
        }

        self.subtask_value = initial;
        if candidate != 0 {
            let flags = callees.candidate_flags(candidate);
            if flags & CANDIDATE_KIND_MASK == CANDIDATE_KIND_SPECIAL
                && callees.candidate_state(candidate) == CANDIDATE_STATE_READY
            {
                self.subtask_value = callees.special_value();
            }
        }

        self.subtask_state = 0;
        self.subtask = candidate;
        callees.construct(candidate);
        self.subtask_status = final_status;
    }
}
