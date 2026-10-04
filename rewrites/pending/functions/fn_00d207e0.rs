// original: 0x00d207e0 store_two_floats_c0
// Stores two single-precision values from the stack into the object at
// +0xC0 and +0xC4. Pure bit moves, no arithmetic; returns nothing meaningful.
export!(thiscall, rw_00d207e0(this_ptr: u32, first: u32, second: u32) -> u32 {
    unsafe {
        const FIRST_OFF: u32 = 0xC0;
        const SECOND_OFF: u32 = 0xC4;
        *((this_ptr.wrapping_add(FIRST_OFF)) as *mut u32) = first;
        *((this_ptr.wrapping_add(SECOND_OFF)) as *mut u32) = second;
        0
    }
});
