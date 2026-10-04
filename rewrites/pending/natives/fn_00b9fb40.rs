// original: 0x00b9fb40 IS_CHAR_IN_CAR
/// Forward (char, car) to the engine test, store its low byte in the return slot.
export!(cdecl, rw_00b9fb40(ctx: u32) -> u32 {
    const RET_SLOT: usize = 0;
    const ARG_ARRAY: usize = 2;
    let c = ctx as *const u32;
    let retp = unsafe { *c.add(RET_SLOT) } as *mut u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let ch = unsafe { *args.add(0) };
    let car = unsafe { *args.add(1) };
    let r: u32 = callee_cdecl!(1, u32, ch, car);
    unsafe { *retp = r & 0xFF; }
    retp as u32
});
