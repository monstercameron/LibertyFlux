// original: 0x00BBAC10 TASK_SWIM_TO_COORD
// TASK_SWIM_TO_COORD: script native handler, cdecl/1 over the call context.
// Forwards script arguments args[0], args[1], args[2], args[3] to one engine function and returns
// whatever that function returned in EAX (the VM ignores it).
lf_rn04_rt::export!(cdecl, rw_00bbac10(ctx: u32) -> u32 {
    let args = unsafe { *((ctx as *const u32).add(2)) as *const u32 };
    let a0 = unsafe { *args };
    let a1 = unsafe { *args.add(1) };
    let a2 = unsafe { *args.add(2) };
    let a3 = unsafe { *args.add(3) };
    lf_rn04_rt::callee_cdecl!(1, u32, a0, a1, a2, a3)
});
