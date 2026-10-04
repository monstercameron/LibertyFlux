// original: 0x00b09e00 store_four_floats_and_flag
/// Store four float words and a flag byte into the camera object.
///
/// Copies the four incoming float bit patterns into their slots and the
/// low byte of the last argument into the flag slot. Pure data move.
export!(thiscall, rw_00b09e00(this_ptr: u32, f0: u32, f1: u32, f2: u32, f3: u32, flag: u32) -> u32 {
    unsafe {
        let base = this_ptr as usize;
        *((base + 0x19c) as *mut u32) = f0;
        *((base + 0x1a8) as *mut u32) = f1;
        *((base + 0x1ac) as *mut u32) = f2;
        *((base + 0x1c0) as *mut u32) = f3;
        *((base + 0x1ec) as *mut u8) = (flag & 0xff) as u8;
        0
    }
});
