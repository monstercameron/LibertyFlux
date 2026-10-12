//! The collaborators of the mirror-reflection render phase, as one trait.
//!
//! Every call the phase's methods make that is not lifted here becomes one
//! method of [`MirrorHost`]: the numbered callees of the original, its
//! virtual slot, and the reads and writes of objects the phase does not own.
//! Arguments keep the original's order, and a method's pointer arguments
//! that point at data the phase builds for the call arrive as slices.

/// A reference to an object, named without an address.
///
/// The phase's own members and the scene snapshot it is given are reached by
/// a byte offset from their base. Every other object is an opaque handle: the
/// lift carries it, never interprets it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ref {
    /// A member of the render-phase object, at this byte offset.
    Phase(u32),
    /// A member of the scene snapshot the phase is given, at this byte offset.
    Snapshot(u32),
    /// An object the original returned or was given.
    Handle(u32),
}

/// A word passed as a value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Word {
    /// A plain 32-bit value.
    Value(u32),
    /// The render-phase identity word with its low byte replaced by this
    /// byte. The original passes the object's address with its low byte
    /// changed; the lift names the value instead of holding the address.
    PhaseTagged(u8),
}

/// Everything the mirror-reflection phase asks of the code around it.
///
/// Reads through [`MirrorHost::word`] and [`MirrorHost::byte`] are reads
/// of an object the phase does not own. The phase's own members are never
/// reached through this trait.
pub trait MirrorHost {
    /// Reads the word at `offset` inside `object`.
    fn word(&mut self, object: Ref, offset: u32) -> u32;
    /// Reads the byte at `offset` inside `object`.
    fn byte(&mut self, object: Ref, offset: u32) -> u8;
    /// Writes the word at `offset` inside `object`.
    fn set_word(&mut self, object: Ref, offset: u32, value: u32);

    /// Destructor of the render-phase object (first slot of the object's
    /// deleting destructor).
    fn destruct(&mut self, this: Ref);
    /// Frees the storage of the render-phase object.
    fn free_storage(&mut self, this: Ref);

    /// Probes the capability at the descriptor, answering a flag word.
    fn probe_gate(&mut self, descriptor: Ref) -> u32;
    /// Answers a query block of a kind, or zero when there is none.
    fn query(&mut self, kind: u32, sub: u32) -> u32;
    /// Applies a query block with six further words.
    fn apply_six(&mut self, query: Ref, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32;
    /// Releases a handle returned by [`MirrorHost::query`] or
    /// [`MirrorHost::apply_six`].
    fn release(&mut self, handle: Ref);
    /// Answers a pointer to four float words.
    fn fetch_floats(&mut self) -> u32;
    /// Feeds a four-float block to a query block.
    fn feed_four(&mut self, query: Ref, flag: u32, block: &mut [u32; 4]) -> u32;
    /// Toggles a query block on or off.
    fn toggle(&mut self, query: Ref, on: u32) -> u32;
    /// Pairs a query block with two words.
    fn pair(&mut self, query: Ref, a: u32, b: u32) -> u32;
    /// Finds the player object, or answers zero.
    fn find_player(&mut self, zero: u32) -> u32;
    /// Selects a target with two words.
    fn select(&mut self, slot: Ref, a: u32, selector: u32) -> u32;
    /// Commits a target.
    fn commit(&mut self, target: Ref) -> u32;
    /// The virtual slot at byte offset 0x30 of `object`, called with `arg`.
    fn slot30(&mut self, object: Ref, arg: Word) -> u32;

    /// Binds a descriptor to a view of the snapshot.
    fn bind(&mut self, descriptor: Ref, view: Ref) -> u32;
    /// Reduces the parameter block into an eight-word frame; the callee
    /// fills the second half of the frame.
    fn reduce(&mut self, params: Ref, selector: u32, frame: &mut [u32; 8]) -> u32;
    /// First describe step on a four-word half frame.
    fn describe(&mut self, half: &mut [u32; 4]) -> u32;
    /// Second describe step, with a descriptor.
    fn describe_with(&mut self, half: &mut [u32; 4], descriptor: Ref) -> u32;
    /// Combines a selector, a block and the half frame.
    fn combine(&mut self, selector: u32, block: Ref, half: &mut [u32; 4], phase: Ref, zero: u32) -> u32;
    /// Finishes the half frame.
    fn finish(&mut self, half: &mut [u32; 4]) -> u32;
    /// Fallback step when the parameter block is not usable.
    fn fallback(&mut self, zero: u32) -> u32;
    /// The current matrix, four words.
    fn current_matrix(&mut self) -> [u32; 4];
    /// Projects a matrix of four words through the phase.
    fn project(&mut self, phase: Ref, matrix: &mut [u32; 4]) -> u32;
    /// The six-word setter on a descriptor.
    fn set_six(&mut self, descriptor: Ref, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32;
    /// A fresh matrix, four words.
    fn fresh_matrix(&mut self) -> [u32; 4];
    /// Projects a scaled matrix through a descriptor.
    fn scale_project(&mut self, descriptor: Ref, matrix: &mut [u32; 4]) -> u32;

    /// Rebuilds the reflection target from three descriptors.
    fn rebuild_target(&mut self, desc_a: Ref, desc_b: Ref, desc_c: Ref, phase: Ref, one: u32, zero: u32);
    /// Fetches the target handle through the session object.
    fn fetch_target(&mut self, session: Ref) -> u32;
    /// Looks up the current target object, or answers zero.
    fn lookup_object(&mut self, zero: u32) -> u32;
    /// Phase footer: the tail of the refresh, its answer is returned.
    fn footer(&mut self) -> u32;
}
