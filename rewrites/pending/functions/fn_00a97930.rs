// original: 0x00a97930 fade_slot_clear
/// Releases the channel's head entry and parks the slot index.
///
/// Hands the head word to the releaser, then zeroes the indexed slot word
/// and the secondary head word. Returns the slot index that was parked.
export!(thiscall, rw_00a97930(this: u32) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, *(this as *const u32));
        let n = *((this + 0x14C) as *const u32);
        *((this.wrapping_add(n.wrapping_mul(8)).wrapping_add(0x10)) as *mut u32) = 0;
        *((this + 8) as *mut u32) = 0;
        n
    }
});
