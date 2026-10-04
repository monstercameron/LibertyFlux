// original: 0x00d698d0 invalidate_inner
// s16f12: invalidate the child record when present (thiscall/0).
//
// Null-checks this record's child pointer, then applies the three reset
// stores the original reaches through a tail jump. Empty child does
// nothing. No meaningful return value.
export!(thiscall, rw_s16f12(this: *const u8) -> () {
    unsafe {
        let child = *((this.add(4)) as *const u32);
        if child == 0 {
            return;
        }
        *((child.wrapping_add(0xA0)) as *mut u32) = 0xFFFFFFFF;
        *((child.wrapping_add(0xF8)) as *mut u32) = 0;
        *((child.wrapping_add(0xFC)) as *mut u32) = 0xFFFFFFFF;
    }
});
