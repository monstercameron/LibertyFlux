// original: 0x00BBA350 TASK_PLAY_ANIM_SECONDARY_UPPER_BODY

/// Forward all nine script arguments to the animation task engine in their
/// original order, preserving each argument as a raw 32-bit word. The fourth
/// argument carries float bits and is forwarded without numeric conversion.
lf_checker_rt::export!(cdecl, rw_00bba350(ctx: u32) -> u32 {
    unsafe {
        let args = (ctx.wrapping_add(8) as *const u32).read_unaligned();
        let a0 = (args as *const u32).read_unaligned();
        let a1 = (args.wrapping_add(4) as *const u32).read_unaligned();
        let a2 = (args.wrapping_add(8) as *const u32).read_unaligned();
        let a3 = (args.wrapping_add(12) as *const u32).read_unaligned();
        let a4 = (args.wrapping_add(16) as *const u32).read_unaligned();
        let a5 = (args.wrapping_add(20) as *const u32).read_unaligned();
        let a6 = (args.wrapping_add(24) as *const u32).read_unaligned();
        let a7 = (args.wrapping_add(28) as *const u32).read_unaligned();
        let a8 = (args.wrapping_add(32) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(1, u32, a0, a1, a2, a3, a4, a5, a6, a7, a8)
    }
});
