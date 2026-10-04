// original: 0x00a015f0 IS_OBJECT_STATIC
/// Forward the object handle to the engine test, store its low byte.
export!(cdecl, rw_00a015f0(ctx: u32) -> u32 {
    const RET_SLOT: usize = 0;
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let retp = unsafe { *c.add(RET_SLOT) } as *mut u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let obj = unsafe { *args.add(0) };
    let r: u32 = callee_cdecl!(1, u32, obj);
    unsafe { *retp = r & 0xFF; }
    retp as u32
});
