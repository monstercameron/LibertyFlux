// original: 0x00BC5EA0 GET_EXTRA_CAR_COLOURS
// GET_EXTRA_CAR_COLOURS: script native handler, cdecl/1 over the call context.
// Forwards script arguments args[0], args[1], args[2] to one engine function and returns
// whatever that function returned in EAX (the VM ignores it).
export!(cdecl, rw_00bc5ea0(ctx: u32) -> u32 {
    let args = unsafe { *((ctx as *const u32).add(2)) as *const u32 };
    let a0 = unsafe { *args };
    let a1 = unsafe { *args.add(1) };
    let a2 = unsafe { *args.add(2) };
    callee_cdecl!(1, u32, a0, a1, a2)
});
