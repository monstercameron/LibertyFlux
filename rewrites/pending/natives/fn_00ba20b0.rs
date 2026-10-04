// original: 0x00BA20B0 SET_GROUP_FORMATION
// SET_GROUP_FORMATION: script native handler, cdecl/1 over the call context.
// Forwards script arguments args[0], args[1] to one engine function and returns
// whatever that function returned in EAX (the VM ignores it).
export!(cdecl, rw_00ba20b0(ctx: u32) -> u32 {
    let args = unsafe { *((ctx as *const u32).add(2)) as *const u32 };
    let a0 = unsafe { *args };
    let a1 = unsafe { *args.add(1) };
    callee_cdecl!(1, u32, a0, a1)
});
