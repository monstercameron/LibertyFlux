// original: 0x00bb8e90 SET_ALTERNATIVE_HEAD_FOR_PED_QUEUE
/// Script native `SET_ALTERNATIVE_HEAD_FOR_PED_QUEUE` (hash 0x5B814784).
///
/// Forwards five script arguments to the engine: a queue index and four
/// float bit-patterns. Floats are copied as raw bits, so the forward is
/// bit-exact. No return slot is written. (The original pre-arranges the
/// arguments in its stack frame, with below-ESP temporaries, instead of
/// pushing them one by one; the observable call is an ordinary forward.)
export!(cdecl, rw_00bb8e90(ctx: *const u8) -> u32 {
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
        )
    }
});
