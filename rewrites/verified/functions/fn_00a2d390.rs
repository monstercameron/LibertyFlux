// original: 0x00a2d390 CPlayerPed::vf81

/// Store a word parameter at `+0xEE0` of the object.
/// Original: 0x00a2d390 (thiscall, one stack word, no return value).
lf_checker_rt::export!(thiscall, rw_00a2d390(this: u32, val: u32) -> u32 {
    unsafe {
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
        const SLOT: u32 = 0xee0;
        wr32(this.wrapping_add(SLOT), val);
        0
    }
});
