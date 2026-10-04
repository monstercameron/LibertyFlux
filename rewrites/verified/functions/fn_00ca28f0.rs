// original: 0x00ca28f0 triple_null_check

/// Report whether all three face slots are populated.
///
/// Returns 1 when the words at `+0x18`, `+0x14` and `+0x10` are all non-zero,
/// else 0. Only the low byte is set; the upper bytes keep their entry value.
///
/// Original: 0x00ca28f0 (thiscall, no stack words, returns al).
lf_checker_rt::export!(thiscall, rw_00ca28f0(this: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe {
        if rd32(this + 0x18) != 0 && rd32(this + 0x14) != 0 && rd32(this + 0x10) != 0 {
            1
        } else {
            0
        }
    }
});
