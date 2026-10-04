// original: 0x00bb2040 GET_TIME_SINCE_LAST_ARREST
/// Call the arrest-timer engine function and store its result in the return slot.
rt::export!(cdecl, rw_00bb2040(ctx: u32) -> u32 {
    const RET_SLOT: usize = 0;
    let c = ctx as *const u32;
    let retp = unsafe { *c.add(RET_SLOT) } as *mut u32;
    let r: u32 = rt::callee_cdecl!(1, u32,);
    unsafe { *retp = r; }
    r
});
