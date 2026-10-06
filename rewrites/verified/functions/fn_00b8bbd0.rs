// original: 0x00b8bbd0 ADD_TICKER_TO_PREVIOUS_BRIEF_WITH_UNDERSCORE
/// Script native `ADD_TICKER_TO_PREVIOUS_BRIEF_WITH_UNDERSCORE`.
///
/// Forwards 7 script arguments to the engine: argument 0 forwarded unchanged; argument 1 coerced to 0/1 in the low byte of a frame temporary (upper bytes unspecified, compared low byte only); argument 2 forwarded unchanged; argument 3 forwarded unchanged; argument 4 coerced to 0/1 in the low byte of a frame temporary (upper bytes unspecified, compared low byte only); argument 5 forwarded unchanged; argument 6 coerced to 0/1 in the low byte of a word whose upper bytes repeat the context pointer.
///
/// No return slot is written; the engine answer is returned.
export!(cdecl, rw_00b8bbd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            u32::from(*args.add(1) != 0),
            *args.add(2),
            *args.add(3),
            u32::from(*args.add(4) != 0),
            *args.add(5),
            (ctx as u32 & 0xFFFF_FF00) | u32::from(*args.add(6) != 0),
        )
    }
});
