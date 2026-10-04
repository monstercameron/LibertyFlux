// original: 0x00bb9600 TASK_DRIVE_BY
/// Script native `TASK_DRIVE_BY` (hash 0x3FB22EE2).
///
/// Forwards ten script arguments to the engine: three leading words (handles
/// and flags), four float bit-patterns, a further word, a boolean flag, and
/// a trailing word. The engine resolves the handles and assigns the task.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The full dword is reproduced here
/// for bit-exact outgoing-call matching. No return slot is written; the
/// engine answer is the exit value.
export!(cdecl, rw_00bb9600(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(8) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
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
            *args.add(7),
            quirked,
            *args.add(9),
        )
    }
});
