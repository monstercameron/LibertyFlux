// original: 0x00684940 frame_state_init
/// Small frame-state initializer: writes the vtable pointer at +0, zeroes
/// the words at +4, +8, +0xC and +0x10, and returns the object pointer.
/// The vtable immediate carries a loader fixup, so it is relocated.
export!(thiscall, rw_00684940(this_: *mut u8) -> u32 {
    unsafe {
        *(this_ as *mut u32) = relocated(0x00FE_385Cu32);
        *(this_.add(4) as *mut u32) = 0;
        *(this_.add(8) as *mut u32) = 0;
        *(this_.add(0x0C) as *mut u32) = 0;
        *(this_.add(VALUE_OFF) as *mut u32) = 0;
        this_ as u32
    }
});
