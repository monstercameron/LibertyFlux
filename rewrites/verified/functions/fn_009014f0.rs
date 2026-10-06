// original: 0x009014f0 input_handle_lookup (proposed)
/// Look up a handle: validate the index and match the serial number.
///
/// `handle` packs a 16-bit serial in the high half and a 16-bit index in
/// the low half. Returns -1 for a handle of -1, an index above `MAX_INDEX`
/// (unsigned `ja` bound check), a null table slot, or a serial that differs
/// from the slot object's first word; otherwise returns the index. Cdecl
/// with one stack word.
export!(cdecl, rw_009014f0(handle: u32) -> u32 {
    unsafe {
        /// Handle table base (file VA).
        const TABLE: u32 = 0x0118F6F8;
        /// Highest valid index, compared unsigned.
        const MAX_INDEX: u32 = 0x5DB;
        if handle == 0xFFFFFFFF {
            return 0xFFFFFFFF;
        }
        let idx = handle & 0xFFFF;
        if idx > MAX_INDEX {
            return 0xFFFFFFFF;
        }
        let slot = ((relocated(TABLE).wrapping_add(idx.wrapping_mul(4))) as *const u32)
            .read_unaligned();
        if slot == 0 {
            return 0xFFFFFFFF;
        }
        let serial = handle >> 16;
        let stored = ((slot.wrapping_add(0)) as *const u16).read_unaligned() as u32;
        if serial != stored {
            return 0xFFFFFFFF;
        }
        idx
    }
});
