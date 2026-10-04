// original: 0x00bd8250 NETWORK_END_SESSION
/// End the network session via the engine function, store its low byte.
export!(cdecl, rw_00bd8250(ctx: u32) -> u32 {
    const RET_SLOT: usize = 0;
    let c = ctx as *const u32;
    let retp = unsafe { *c.add(RET_SLOT) } as *mut u32;
    let r: u32 = callee_cdecl!(1, u32,);
    unsafe { *retp = r & 0xFF; }
    retp as u32
});
