// original: 0x00bc4ff0 ADD_STUCK_CAR_CHECK_WITH_WARP
/// Script native `ADD_STUCK_CAR_CHECK_WITH_WARP`.
///
/// Forwards 7 script arguments to the engine: argument 0 forwarded unchanged; argument 1 forwarded as raw bits; argument 2 forwarded unchanged; argument 3 coerced to 0/1 in the low byte of a frame temporary (upper bytes unspecified, compared low byte only); argument 4 coerced to 0/1 in the low byte of a frame temporary (upper bytes unspecified, compared low byte only); argument 5 coerced to 0/1 in the low byte of a word whose upper bytes repeat the context pointer; argument 6 forwarded unchanged.
///
/// No return slot is written; the engine answer is returned.
export!(cdecl, rw_00bc4ff0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            u32::from(*args.add(3) != 0),
            u32::from(*args.add(4) != 0),
            (ctx as u32 & 0xFFFF_FF00) | u32::from(*args.add(5) != 0),
            *args.add(6),
        )
    }
});
