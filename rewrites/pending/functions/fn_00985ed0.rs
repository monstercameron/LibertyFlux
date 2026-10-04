// original: 0x00985ed0 audStaticRadioEmitter::vf12
/// Copy the emitter's two id words to the out-pointers.
///
/// Stores `this+0x38` through `o1` and `this+0x3c` through `o2`, and
/// returns the second out-pointer.
export!(thiscall, rw_00985ed0(this: u32, o1: u32, o2: u32) -> u32 {
    unsafe {
        *(o1 as *mut u32) = *((this.wrapping_add(0x38)) as *const u32);
        *(o2 as *mut u32) = *((this.wrapping_add(0x3c)) as *const u32);
        o2
    }
});
