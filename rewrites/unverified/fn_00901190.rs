// original: 0x00901190 input_handle_alloc_id (proposed)
/// Stamp a live table slot with the next serial id and build its handle.
///
/// `idx` selects a slot of the handle table; a null slot returns -1.
/// Otherwise the static serial word is advanced (wrapping from `0xFFFE` or
/// `0xFFFF` back to 1, unsigned `jae` compare), stored both to the static
/// word and to the slot object's first halfword, and the new handle
/// `(serial << 16) | (idx & 0xFFFF)` is returned. Cdecl, one stack word.
export!(cdecl, rw_00901190(idx: u32) -> u32 {
    unsafe {
        /// Handle table base (file VA).
        const TABLE: u32 = 0x0118F6F8;
        /// Static serial word (file VA).
        const SERIAL: u32 = 0x0118F4DC;
        /// Serial value that wraps back to 1 (compared unsigned).
        const WRAP_AT: u16 = 0xFFFE;
        let slot = ((relocated(TABLE).wrapping_add((idx & 0xFFFFFFFF).wrapping_mul(4)))
            as *const u32)
            .read_unaligned();
        if slot == 0 {
            return 0xFFFFFFFF;
        }
        let cur = (global::<u16>(SERIAL)).read_unaligned();
        let next: u16 = if cur >= WRAP_AT { 1 } else { cur.wrapping_add(1) };
        (global::<u16>(SERIAL)).write_unaligned(next);
        ((slot.wrapping_add(0)) as *mut u16).write_unaligned(next);
        let back = ((slot.wrapping_add(0)) as *const u16).read_unaligned() as u32;
        (back << 16) | (idx & 0xFFFF)
    }
});
