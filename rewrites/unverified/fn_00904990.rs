// original: 0x00904990 input_slot_flag_byte (proposed)
/// Read a slot's flag byte, falling back to the default slot when unset.
///
/// `idx` selects a slot of the handle table. When the slot object's kind
/// byte at `+8` is zero the index in the static default word is used
/// instead. Returns the flag byte at `+0x58` of the chosen object. Cdecl
/// with one stack word.
export!(cdecl, rw_00904990(idx: u32) -> u32 {
    unsafe {
        /// Handle table base (file VA).
        const TABLE: u32 = 0x0118F6F8;
        /// Static default-slot index word (file VA).
        const DEFAULT: u32 = 0x01034494;
        const KIND_OFF: u32 = 0x08;
        const FLAG_OFF: u32 = 0x58;
        let mut obj = ((relocated(TABLE).wrapping_add(idx.wrapping_mul(4))) as *const u32)
            .read_unaligned();
        if ((obj.wrapping_add(KIND_OFF)) as *const u8).read() == 0 {
            let d = (global::<u32>(DEFAULT)).read_unaligned();
            obj = ((relocated(TABLE).wrapping_add(d.wrapping_mul(4))) as *const u32)
                .read_unaligned();
        }
        ((obj.wrapping_add(FLAG_OFF)) as *const u8).read() as u32
    }
});
