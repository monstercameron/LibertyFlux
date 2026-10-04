// original: 0x00bb95c0 TASK_DEAD
/// Forward the char handle to the death-task engine function.
export!(cdecl, rw_00bb95c0(ctx: u32) -> u32 {
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let ch = unsafe { *args.add(0) };
    callee_cdecl!(1, u32, ch)
});
