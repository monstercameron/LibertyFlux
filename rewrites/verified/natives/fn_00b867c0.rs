// original: 0x00B867C0 CAM_SEQUENCE_OPEN
// CAM_SEQUENCE_OPEN: script native handler, cdecl/1 over the call context.
// Forwards script argument args[0] to one engine function and returns
// whatever that function returned in EAX (the VM ignores it).
export!(cdecl, rw_00b867c0(ctx: u32) -> u32 {
    let args = unsafe { *((ctx as *const u32).add(2)) as *const u32 };
    let a0 = unsafe { *args };
    callee_cdecl!(1, u32, a0)
});
