// original: 0x0088acc0 audio_slot_lookup_store
/// Resolves a position value to a slot byte and stores it.
///
/// When `value` is null, stores `0xFF` at `this+index+0x48`. Otherwise
/// subtracts the row base (from the global entity table selected by byte
/// `this+0x40`) from `value`, divides by the global stride, and stores the
/// low byte of the quotient at `this+index+0x48`. Returns `index`.
export!(thiscall, rw_0088acc0(this_ptr: *mut u8, index: u32, value: u32) -> u32 {
    unsafe {
        if value == 0 {
            *this_ptr.add(index.wrapping_add(0x48) as usize) = 0xFF;
            return index;
        }
        let base = *(relocated(0x0115D988) as *const u32) as *const u8;
        let stride = *(relocated(0x0115D964) as *const u32);
        let row_sel = *this_ptr.add(0x40) as u32;
        let row_base = *(base.add(row_sel.wrapping_mul(0x6F40).wrapping_add(0x6F10) as usize)
            as *const u32);
        let quot = value.wrapping_sub(row_base) / stride;
        *this_ptr.add(index.wrapping_add(0x48) as usize) = quot as u8;
        index
    }
});
