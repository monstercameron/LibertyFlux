// original: 0x00bb6380 GET_INT_STAT
/// Script native `GET_INT_STAT` (hash 0x48994D58).
///
/// Forwards one script argument (a statistic index) to the engine and
/// stores all 32 bits of its answer into the return slot.
export!(cdecl, rw_00bb6380(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer: u32 = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
