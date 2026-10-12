//! What the sound base class needs from the engine around it.
//!
//! Every global the methods read, every numbered callee and every virtual
//! slot they call is one method here. Each method's parameters are the
//! callee's arguments in the order the original pushes them, so a call
//! recorded by a fake world can be compared argument by argument with the
//! verified rewrite's call. Objects that the original reaches by address
//! (the sound itself, voice objects, link targets, slot words) are opaque
//! handles.

use lf_core::Handle32;

/// Tag for an opaque object that carries a virtual table: a sound, a voice
/// object found through a lookup, or a link target.
pub struct ObjectTag;
/// Tag for the sound registry cell that the registration callees take.
pub struct RegistryTag;
/// Tag for the global fallback object cell that a link without a voice
/// forwards to.
pub struct FallbackTag;
/// Tag for a word or byte in engine memory that the class writes through
/// a pointer it was given (a lookup answer or a stored slot pointer).
pub struct SlotTag;

/// The engine side of the sound base class.
///
/// Globals are read through the accessors; the numbered callees and the
/// virtual slots are called with the original's arguments, in order.
pub trait SoundWorld {
    /// Scalar global used as the id stride of the voice cursor table in
    /// the lookup and as the link stride in the forwarding method.
    fn scalar_a(&self) -> u32;
    /// Scalar global used as the kind stride of the initialisation cursor.
    fn scalar_b(&self) -> u32;
    /// The word at `offset` inside the voice-table row for `row` (a row
    /// base of `row * ROW_STRIDE` past the table global). Raw data: the
    /// table holds every row's words, so this is a read, not an address.
    fn voice_table_word(&self, row: u8, offset: u32) -> u32;
    /// The sound registry cell.
    fn registry(&self) -> Handle32<RegistryTag>;
    /// The global fallback object that link-less sounds forward to.
    fn fallback_cell(&self) -> Handle32<FallbackTag>;
    /// Stores the class's virtual table into the object's first word.
    fn install_vtable(&mut self, this: Handle32<ObjectTag>);
    /// Runs the initialisation entry for `sub_id` from the init table,
    /// with the object as its only argument (indirect, cdecl).
    fn run_init(&mut self, this: Handle32<ObjectTag>, sub_id: u8) -> u32;
    /// Numbered callee 2: hands the derived cursor to the cursor entry
    /// (thiscall, one argument; the cursor is a raw word and may be any
    /// value).
    fn set_cursor(&mut self, cursor: u32) -> u32;
    /// Numbered callee 3: registers a sound with the registry, passing the
    /// registry cell, the voice id and the kind (thiscall, three arguments).
    fn register(&mut self, registry: Handle32<RegistryTag>, voice_id: u32, kind: u32) -> u32;
    /// Numbered callee 1 of the hashing routines: the name hash, cdecl with
    /// the name and a seed.
    fn joaat(&mut self, name: u32, seed: u32) -> u32;
    /// Virtual slot 0 of an object: the lookup by hash (thiscall, object
    /// and hash). The answer is a raw pointer, zero when nothing matched.
    fn find_slot0(&mut self, object: Handle32<ObjectTag>, hash: u32) -> u32;
    /// Virtual slot 2 of an object (thiscall, object, hash and the float
    /// bits).
    fn slot2(&mut self, object: Handle32<ObjectTag>, hash: u32, value_bits: u32) -> u32;
    /// Virtual slot 4 of a link target (thiscall, target and argument).
    /// The target is the raw word the table yields; the original faults on
    /// a null target, so it is never zero on a completed call.
    fn slot4(&mut self, target: u32, arg: u32) -> u32;
    /// Numbered callee 1 of the forwarding method: the fallback cell and the
    /// argument (thiscall, two arguments).
    fn forward_fallback(&mut self, cell: Handle32<FallbackTag>, arg: u32) -> u32;
    /// Writes a word through a pointer the lookup returned.
    fn store_word(&mut self, slot: Handle32<SlotTag>, value: u32);
    /// Writes a zero byte through a stored slot pointer.
    fn zero_byte(&mut self, slot: Handle32<SlotTag>);
    /// Writes a zero word through a stored slot pointer.
    fn zero_word(&mut self, slot: Handle32<SlotTag>);
    /// Numbered callee 1 of the deleting destructor: the second constructor
    /// for `this` (thiscall, one argument).
    fn rebuild(&mut self, this: Handle32<ObjectTag>) -> u32;
    /// Numbered callee 2 of the deleting destructor: unregisters `this`
    /// with its voice id from the registry (thiscall, three arguments).
    fn unregister(&mut self, registry: Handle32<RegistryTag>, this: Handle32<ObjectTag>, voice_id: u32) -> u32;
}
