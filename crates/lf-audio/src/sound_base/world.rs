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
/// Tag for the scratch frame the attach callees write through.
pub struct FrameTag;

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
    fn unregister(
        &mut self,
        registry: Handle32<RegistryTag>,
        this: Handle32<ObjectTag>,
        voice_id: u32,
    ) -> u32;

    /// The word at the start of an attach descriptor.
    fn descriptor_word(&self, desc: u32) -> u32;
    /// The half-word at byte 4 of an attach descriptor: its low byte is the
    /// voice id and its high byte the option.
    fn descriptor_pair(&self, desc: u32) -> u16;
    /// The flag byte of an attach descriptor (byte 5).
    fn descriptor_flag(&self, desc: u32) -> u8;
    /// The first byte of a voice record (its sub-id).
    fn voice_sub_id(&self, voice: u32) -> u8;
    /// The word an extra argument points at.
    fn extra_word(&self, extra: u32) -> u32;
    /// The scratch frame that the set-up callees write through.
    fn scratch_frame(&mut self) -> Handle32<FrameTag>;
    /// Numbered callee 1 of attach: initialises the scratch frame
    /// (thiscall, one argument).
    fn init_block(&mut self, frame: Handle32<FrameTag>) -> u32;
    /// Numbered callee 2 of attach: binds the voice to the sound
    /// (thiscall, four arguments: the sound, the voice, a zero and the
    /// frame).
    fn attach_callee(
        &mut self,
        this: Handle32<ObjectTag>,
        voice: u32,
        zero: u32,
        frame: Handle32<FrameTag>,
    ) -> u32;
    /// Numbered callee 3 of attach: binds the extra argument to the sound
    /// (thiscall, three arguments: the sound, the extra word and the frame).
    fn bind_extra(
        &mut self,
        this: Handle32<ObjectTag>,
        extra: u32,
        frame: Handle32<FrameTag>,
    ) -> u32;
    /// Numbered callee 4 of attach: resets the sound (thiscall, one argument).
    fn reset_object(&mut self, this: Handle32<ObjectTag>) -> u32;
    /// The byte at `offset` of a voice record. The record is a raw word:
    /// the table yields it, and the original reads it through the pointer.
    fn record_byte(&self, record: u32, offset: u32) -> u8;
    /// Ors `mask` into the byte at `offset` of a voice record.
    fn or_record_byte(&mut self, record: u32, offset: u32, mask: u8);
    /// Writes a byte at `offset` of a voice record.
    fn set_record_byte(&mut self, record: u32, offset: u32, value: u8);
    /// The word at `offset` of a voice record.
    fn record_word(&self, record: u32, offset: u32) -> u32;
    /// Writes a word at `offset` of a voice record.
    fn set_record_word(&mut self, record: u32, offset: u32, value: u32);
    /// One word of the divisor table for a lane: `slot` 0 is the lane's
    /// rotation counter, `slot` 1 the word after it. The table is raw data.
    fn divisor(&self, lane: u32, slot: u32) -> u32;
    /// The memoised-hash flag word.
    fn memo_flag(&self) -> u32;
    /// Stores the memoised-hash flag word.
    fn set_memo_flag(&mut self, flag: u32);
    /// The memoised category hash.
    fn memo_value(&self) -> u32;
    /// Stores the memoised category hash.
    fn set_memo_value(&mut self, value: u32);
    /// Numbered callee 5 of attach: hashes the category key constant with a
    /// zero seed (cdecl, two arguments).
    fn category_key_hash(&mut self) -> u32;
    /// Numbered callee 6 of attach: looks a category up in the category
    /// registry (thiscall, the registry cell and the category).
    fn category_lookup(&mut self, category: u32) -> u32;
    /// Numbered callee 7 of attach: asks whether a looked-up category
    /// accepts the sound (thiscall, one argument).
    fn category_ok(&mut self, found: u32) -> u32;
}
