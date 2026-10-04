// original: 0x00bc5db0 GET_CURRENT_POLICE_CAR_MODEL
/// Script native `GET_CURRENT_POLICE_CAR_MODEL` (hash 0x20A53B7F).
///
/// Forwards one script argument to the engine. Writes no return slot; the engine answer is only returned in EAX.
export!(cdecl, rw_00bc5db0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0))
    }
});
