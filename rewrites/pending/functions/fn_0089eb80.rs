// original: 0x0089EB80 rage::audStreamingSound::audStreamingSound
// ---------------------------------------------------------------------------
// 0x0089EB80 rage::audStreamingSound constructor: run the base sound
// constructor, then stamp the vtable pointer and the default field values.
// Returns this.
// ---------------------------------------------------------------------------
export!(thiscall, rw_0089EB80(this_ptr: u32) -> u32 {
    callee_thiscall!(1, u32, this_ptr);
    unsafe {
        *((this_ptr.wrapping_add(0xDC)) as *mut u16) = 0xFFFF;
        *((this_ptr) as *mut u32) = relocated(0xE7A454);
        *((this_ptr.wrapping_add(0xB8)) as *mut u32) = 0;
        *((this_ptr.wrapping_add(0xBC)) as *mut u32) = 0;
        *((this_ptr.wrapping_add(0xC0)) as *mut u32) = 0;
        *((this_ptr.wrapping_add(0xC4)) as *mut u32) = 0;
        *((this_ptr.wrapping_add(0xDF)) as *mut u32) = 0x100;
        *((this_ptr.wrapping_add(0xCC)) as *mut u32) = 0;
        *((this_ptr.wrapping_add(0xE3)) as *mut u8) = 0;
        *((this_ptr.wrapping_add(0xDE)) as *mut u8) = 0;
        *((this_ptr.wrapping_add(0xD8)) as *mut u32) = 0;
        *((this_ptr.wrapping_add(0xB0)) as *mut u32) = 0xFFFF_FFFF;
        *((this_ptr.wrapping_add(0xB4)) as *mut u32) = 0xFFFF_FFFF;
    }
    this_ptr
});
