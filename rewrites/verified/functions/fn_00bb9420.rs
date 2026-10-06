// original: 0x00bb9420 TASK_CHAT_WITH_CHAR
/// Script native `TASK_CHAT_WITH_CHAR`.
///
/// Forwards 4 script arguments to the engine: argument 0 forwarded unchanged; argument 1 forwarded unchanged; argument 2 coerced to 0/1 in the low byte of a caller-register temporary (upper bytes unspecified, compared low byte only); argument 3 coerced to 0/1 in the low byte of a word whose upper bytes repeat the context pointer.
///
/// No return slot is written; the engine answer is returned.
export!(cdecl, rw_00bb9420(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            u32::from(*args.add(2) != 0),
            (ctx as u32 & 0xFFFF_FF00) | u32::from(*args.add(3) != 0),
        )
    }
});
