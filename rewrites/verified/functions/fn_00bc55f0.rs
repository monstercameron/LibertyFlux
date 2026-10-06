// original: 0x00bc55f0 DISABLE_CAR_GENERATORS
/// Script native `DISABLE_CAR_GENERATORS`.
///
/// Forwards 2 script arguments to the engine: argument 0 coerced to 0/1 in the low byte of a caller-register temporary (upper bytes unspecified, compared low byte only); argument 1 coerced to 0/1 in the low byte of a word whose upper bytes repeat the context pointer.
///
/// No return slot is written; the engine answer is returned.
export!(cdecl, rw_00bc55f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, u32::from(*args != 0), (ctx as u32 & 0xFFFF_FF00) | u32::from(*args.add(1) != 0))
    }
});
