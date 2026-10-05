// original: 0x00b8db40 START_GPS_RACE_TRACK
/// Script native `START_GPS_RACE_TRACK` (hash 0x422C1818).
///
/// Forwards one script argument (a track id) to the engine. No return slot
/// is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b8db40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
