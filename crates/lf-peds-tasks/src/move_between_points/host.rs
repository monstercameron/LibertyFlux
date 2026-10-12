//! The collaborators and callees of the move-between-points scenario.
//!
//! Each method here is something a lifted scenario method calls that is not
//! lifted itself, or a sub-task the scenario hands work to. The lifted methods
//! take an implementation of these traits, so a caller decides what happens
//! behind each one; the differential proof records every call in order with its
//! arguments and checks the sequence against the verified rewrite.
//!
//! Handles are opaque 32-bit words, as the original stores them. Zero means no
//! object. A word read from shared game state is returned by a method rather
//! than stored here, so the lift never assumes what the shared state holds.

use super::scenario::MoveBetweenPointsScenario;

/// The services a scenario calls: the base class destructor, the heap, the
/// scenario constructor, and the release of a payload handle.
pub trait MoveBetweenPointsHost {
    /// The heap object word held in shared game state. Read, not called; the
    /// lifted methods read it after the class destructor has run.
    fn heap_singleton(&mut self) -> u32;

    /// The base class destructor, run on the scenario before it is freed.
    fn class_destructor(&mut self, task: &mut MoveBetweenPointsScenario);

    /// Frees the scenario through the heap object.
    fn heap_free(&mut self, heap: u32, task: &mut MoveBetweenPointsScenario);

    /// Allocates a slot for a new scenario from the heap object. Zero when the
    /// allocation fails.
    fn heap_alloc(&mut self, heap: u32) -> u32;

    /// Constructs a scenario in `slot` from the three words the source
    /// scenario carries. Returns the word the constructor returns.
    fn construct(&mut self, slot: u32, aux_id: u32, aux_word: u32, pending_guard: u32) -> u32;

    /// Releases a payload handle that the scenario still holds.
    fn release_payload(&mut self, payload: u32);
}

/// A sub-task the scenario runs each tick. Its latch byte belongs to the
/// sub-task, and `run` is its own run operation.
pub trait SubTask {
    /// The latch byte. Bit 0 says the sub-task has already been accepted.
    fn latch(&self) -> u8;

    /// Replaces the latch byte.
    fn set_latch(&mut self, value: u8);

    /// Runs the sub-task with three arguments. The result is read from its
    /// low byte: zero means the sub-task did not complete this tick.
    fn run(&mut self, a1: u32, a2: u32, a3: u32) -> u32;
}
