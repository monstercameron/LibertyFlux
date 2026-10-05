// original: 0x00bb6540 PLAYSTATS_MISSION_PASSED
/// Script native `PLAYSTATS_MISSION_PASSED` (hash 0x437D3E19).
///
/// Forwards one script argument (the mission identifier) to the engine
/// statistics sender. No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bb6540(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
