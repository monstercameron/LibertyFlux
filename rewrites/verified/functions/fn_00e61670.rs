// original: 0x00e61670 timing_slots_clear_all
/// Invalidate all 128 timing slots, returning the end of the table.
///
/// Each 8-byte slot gets an invalid marker word and a cleared flag word;
/// the two padding bytes of each slot are left untouched.
export!(cdecl, rw_00e61670() -> u32 {
    unsafe {
        let mut ptr = relocated(0x01A01DFC);
        for _ in 0..128u32 {
            *(ptr as *mut u32) = 0xFFFF_FFFF;
            *((ptr + 4) as *mut u16) = 0;
            ptr = ptr.wrapping_add(8);
        }
        ptr
    }
});
