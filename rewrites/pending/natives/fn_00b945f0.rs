// original: 0x00b945f0 GET_MISSION_FLAG
/// Call the mission-flag engine function and store its low byte in the return slot.
rt::export!(cdecl, rw_00b945f0(ctx: u32) -> u32 {
    const RET_SLOT: usize = 0;
    let c = ctx as *const u32;
    let retp = unsafe { *c.add(RET_SLOT) } as *mut u32;
    let r: u32 = rt::callee_cdecl!(1, u32,);
    unsafe { *retp = r & 0xFF; }
    retp as u32
});
