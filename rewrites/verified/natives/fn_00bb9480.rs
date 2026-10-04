// original: 0x00BB9480 TASK_CLIMB_LADDER
//
// Forwards the character handle plus a boolean-ised flag with the
// incoming-slot graffiti (measured offset, checked each trial).
export!(cdecl, rw_00bb9480(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        let flag = (*args.add(1) != 0) as u8;
        let coerced = flag as u32;
        callee_cdecl!(1, u32, *args, coerced)
    }
});

