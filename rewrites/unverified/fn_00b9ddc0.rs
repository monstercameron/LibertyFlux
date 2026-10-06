// original: 0x00b9ddc0 ADD_CHAR_DECISION_MAKER_EVENT_RESPONSE
/// Script native `ADD_CHAR_DECISION_MAKER_EVENT_RESPONSE`.
///
/// Forwards 9 script arguments to the engine: argument 0 forwarded unchanged; argument 1 forwarded unchanged; argument 2 forwarded unchanged; argument 3 forwarded as raw bits; argument 4 forwarded as raw bits; argument 5 forwarded as raw bits; argument 6 forwarded as raw bits; argument 7 coerced to 0/1 in the low byte of a caller-register temporary (upper bytes unspecified, compared low byte only); argument 8 coerced to 0/1 in the low byte of a word whose upper bytes repeat the context pointer.
///
/// No return slot is written; the engine answer is returned.
export!(cdecl, rw_00b9ddc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            *args.add(6),
            u32::from(*args.add(7) != 0),
            (ctx as u32 & 0xFFFF_FF00) | u32::from(*args.add(8) != 0),
        )
    }
});
