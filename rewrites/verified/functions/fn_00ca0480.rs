// original: 0x00ca0480 CTaskSimpleIK::vf3

/// Clear the IK task's active flag and its cached target.
///
/// Clears bit 0 of the flag word at `+0x5c` and zeroes the word at `+0x50`.
/// The single stack word is ignored. No value is returned.
///
/// Original: 0x00ca0480 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ca0480(this: u32, _ignored: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        const FLAGS: u32 = 0x5c;
        const TARGET: u32 = 0x50;
        const ACTIVE_BIT: u32 = 0x0000_0001;
        wr32(this + FLAGS, rd32(this + FLAGS) & !ACTIVE_BIT);
        wr32(this + TARGET, 0);
        0
    }
});
