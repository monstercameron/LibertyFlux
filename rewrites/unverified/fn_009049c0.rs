// original: 0x009049c0 input_slot_mode_word (proposed)
/// Read a slot's mode word, falling back to the default slot when unset.
///
/// `idx` selects a slot of the handle table. When the slot object's kind
/// byte at `+8` is zero the index in the static default word is used
/// instead. Returns the mode word at `+0x54` of the chosen object. Cdecl
/// with one stack word.
export!(cdecl, rw_009049c0(idx: u32) -> u32 {
    unsafe {
        /// Handle table base (file VA).
        const TABLE: u32 = 0x0118F6F8;
        /// Static default-slot index word (file VA).
        const DEFAULT: u32 = 0x01034494;
        const KIND_OFF: u32 = 0x08;
        const MODE_OFF: u32 = 0x54;
        let mut obj = ((relocated(TABLE).wrapping_add(idx.wrapping_mul(4))) as *const u32)
            .read_unaligned();
        if ((obj.wrapping_add(KIND_OFF)) as *const u8).read() == 0 {
            let d = (global::<u32>(DEFAULT)).read_unaligned();
            obj = ((relocated(TABLE).wrapping_add(d.wrapping_mul(4))) as *const u32)
                .read_unaligned();
        }
        ((obj.wrapping_add(MODE_OFF)) as *const u32).read_unaligned()
    }
});
