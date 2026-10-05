// original: 0x00c80eb0 CTaskComplexScenario::vf23

/// Ask the child whether it accepts control (kind `0xdd`), normalised to 0/1.
///
/// Calls virtual slot `+0x3c` with argument `0xdd` and folds the result to a
/// boolean with the original's negate/borrow/negate sequence (0 stays 0,
/// anything else becomes 1). EAX is defined on every path.
///
/// Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00c80eb0(this: u32) -> u32 {
    unsafe {
        const VT_SLOT: u32 = 0x3c;
        const ASK_ARG: u32 = 0xdd;
        let vt = (this as *const u32).read_unaligned();
        let hook: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((vt + VT_SLOT) as *const u32).read_unaligned() as usize,
        );
        u32::from(hook(this, ASK_ARG) != 0)
    }
});
