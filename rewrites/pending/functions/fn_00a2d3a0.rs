// original: 0x00a2d3a0 CPlayerPed::vf79

/// Copy a four-word descriptor into the object at `+0xEB0`.
/// Copies words 0..3 (two of them usually floats, but the copy is by
/// bits) from the stack pointer argument.
/// Original: 0x00a2d3a0 (thiscall, one stack word, no return value).
lf_checker_rt::export!(thiscall, rw_00a2d3a0(this: u32, src: u32) -> u32 {
    unsafe {
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
        const DST: u32 = 0xeb0;
        wr32(this.wrapping_add(DST), rd32(src));
        wr32(this.wrapping_add(DST + 4), rd32(src.wrapping_add(4)));
        wr32(this.wrapping_add(DST + 8), rd32(src.wrapping_add(8)));
        wr32(this.wrapping_add(DST + 12), rd32(src.wrapping_add(12)));
        0
    }
});
