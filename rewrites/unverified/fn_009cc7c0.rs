// original: 0x009cc7c0 START_MOBILE_PHONE_CALL
/// Script native `START_MOBILE_PHONE_CALL`.
///
/// Forwards 6 script arguments to the engine: argument 0 forwarded unchanged; argument 1 forwarded unchanged; argument 2 forwarded unchanged; argument 3 forwarded unchanged; argument 4 coerced to 0/1 in the low byte of a caller-register temporary (upper bytes unspecified, compared low byte only); argument 5 coerced to 0/1 in the low byte of a word whose upper bytes repeat the context pointer.
///
/// No return slot is written; the engine answer is returned.
export!(cdecl, rw_009cc7c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            u32::from(*args.add(4) != 0),
            (ctx as u32 & 0xFFFF_FF00) | u32::from(*args.add(5) != 0),
        )
    }
});
