// original: 0x00b94520 GET_HASH_KEY
/// Hash the string key via the engine function and store it in the return slot.
rt::export!(cdecl, rw_00b94520(ctx: u32) -> u32 {
    const RET_SLOT: usize = 0;
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let retp = unsafe { *c.add(RET_SLOT) } as *mut u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let key = unsafe { *args.add(0) };
    let h: u32 = rt::callee_cdecl!(1, u32, key);
    unsafe { *retp = h; }
    h
});
