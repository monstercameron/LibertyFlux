// original: 0x00a2cc10 ped_floor_ensure

/// Ensure the floor field holds at least 2.0.
/// Reads the float at `+0xEDC`; when it is at or below +0.0 (NaN counts
/// as above and is left alone) the field is set to the 2.0 bit pattern.
/// Original: 0x00a2cc10 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00a2cc10(this: u32) -> u32 {
    unsafe {
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe fn rdf(a: u32) -> f32 {
        unsafe { f32::from_bits(rd32(a)) }
    }
        const FLOOR: u32 = 0xedc;
        const TWO_BITS: u32 = 0x40000000;
        if rdf(this.wrapping_add(FLOOR)) <= 0.0 {
            wr32(this.wrapping_add(FLOOR), TWO_BITS);
        }
        0
    }
});
