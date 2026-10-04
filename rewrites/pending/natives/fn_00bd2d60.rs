// original: 0x00bd2d60 GET_NAME_OF_INFO_ZONE
/// Script native `GET_NAME_OF_INFO_ZONE` (hash 0x5CAD7949).
///
/// Forwards three float coordinates to the engine and stores the full
/// 32-bit engine answer (a zone-name pointer) into the return slot. The
/// coordinates are copied as raw bits, so the forward is bit-exact.
export!(cdecl, rw_00bd2d60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
