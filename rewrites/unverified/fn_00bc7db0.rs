// original: 0x00bc7db0 SET_UPSIDEDOWN_CAR_NOT_DAMAGED
/// Script native `SET_UPSIDEDOWN_CAR_NOT_DAMAGED`.
///
/// Forwards 2 script arguments to the engine: argument 0 forwarded unchanged; argument 1 coerced to 0/1 in the low byte of a word whose upper bytes repeat the context pointer.
///
/// No return slot is written; the engine answer is returned.
export!(cdecl, rw_00bc7db0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, (ctx as u32 & 0xFFFF_FF00) | u32::from(*args.add(1) != 0))
    }
});
