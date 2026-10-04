// original: 0x00bb99c0 TASK_GOTO_CAR
/// Script native `TASK_GOTO_CAR` (hash 0x3EA116F7).
///
/// Forwards four script arguments to the engine: two handles, an integer and
/// one float bit-pattern (a speed or distance) moved through a vector
/// register. The float is forwarded as a raw `u32` bit-pattern so it matches
/// bit-exactly by construction. No return slot is written.
export!(cdecl, rw_00bb99c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
