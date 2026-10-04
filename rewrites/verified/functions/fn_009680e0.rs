// original: 0x009680E0 alloc_timer_slot
/// Allocate the first free slot in the eight-bit mask owned by `this`.
///
/// The mask dword lives behind the pointer at `this + 0x31D0`. Scans bits
/// 0..8; the first clear bit is set and its index returned. Returns -1
/// when all eight bits are set. Only the low byte of the mask dword is
/// ever consulted.
///
/// Original: 0x009680E0 (thiscall, no stack words).

export!(thiscall, rw_009680E0(this: u32) -> u32 {
    unsafe {
        let mask_ptr = ((this.wrapping_add(0x31D0)) as *const u32).read_unaligned();
        let mask = (mask_ptr as *const u32).read_unaligned();
        for i in 0..8u32 {
            if mask & (1u32 << i) == 0 {
                (mask_ptr as *mut u32).write_unaligned(mask | (1u32 << i));
                return i;
            }
        }
        0xFFFF_FFFF
    }
});
