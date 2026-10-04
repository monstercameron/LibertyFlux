// original: 0x00A00630 CLEAR_OBJECT_LAST_DAMAGE_ENTITY
// CLEAR_OBJECT_LAST_DAMAGE_ENTITY: script native handler, cdecl/1 over the call context.
// Forwards script argument args[0] to one engine function and returns
// whatever that function returned in EAX (the VM ignores it).
lf_rn04_rt::export!(cdecl, rw_00a00630(ctx: u32) -> u32 {
    let args = unsafe { *((ctx as *const u32).add(2)) as *const u32 };
    let a0 = unsafe { *args };
    lf_rn04_rt::callee_cdecl!(1, u32, a0)
});
