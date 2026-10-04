// original: 0x00BB8E50 PED_QUEUE_REJECT_PEDS_WITH_FLAG_FALSE
// PED_QUEUE_REJECT_PEDS_WITH_FLAG_FALSE: script native handler, cdecl/1 over the call context.
// Forwards script argument args[0] to one engine function and returns
// whatever that function returned in EAX (the VM ignores it).
lf_rn04_rt::export!(cdecl, rw_00bb8e50(ctx: u32) -> u32 {
    let args = unsafe { *((ctx as *const u32).add(2)) as *const u32 };
    let a0 = unsafe { *args };
    lf_rn04_rt::callee_cdecl!(1, u32, a0)
});
