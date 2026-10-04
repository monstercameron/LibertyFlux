// original: 0x0086e870 TIMERB
/// Read the script B-timer through the global timer pointer into the return slot.
///
/// Leaf handler: no engine call. The dword at file VA 0x01BB54DC points at
/// the timer block; the B timer is the dword at byte offset 0x20 of it.
export!(cdecl, rw_0086e870(ctx: u32) -> u32 {
    const RET_SLOT: usize = 0;
    const TIMER_BLOCK_PTR: u32 = 0x01BB54DC;
    const TIMER_B_WORD: usize = 8;
    let c = ctx as *const u32;
    let retp = unsafe { *c.add(RET_SLOT) } as *mut u32;
    let block = unsafe { *global::<u32>(TIMER_BLOCK_PTR) };
    let v = unsafe { *((block as *const u32).add(TIMER_B_WORD)) };
    unsafe { *retp = v; }
    v
});
