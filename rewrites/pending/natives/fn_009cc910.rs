// original: 0x009cc910 TRIGGER_POLICE_REPORT
/// Script native `TRIGGER_POLICE_REPORT` (hash 0x78D01893).
///
/// Forwards one script argument (a crime string id) to the engine. No return slot is written.
export!(cdecl, rw_009cc910(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
