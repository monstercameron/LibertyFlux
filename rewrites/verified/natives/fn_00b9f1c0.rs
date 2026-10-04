// original: 0x00b9f1c0 GET_CURRENT_BASIC_COP_MODEL
/// Script native `GET_CURRENT_BASIC_COP_MODEL` (hash 0x1B305900).
///
/// Forwards one script argument (an out-pointer) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b9f1c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args,)
    }
});
