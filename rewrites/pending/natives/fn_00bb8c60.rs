// original: 0x00bb8c60 GET_SCRIPT_TASK_STATUS
/// Script native `GET_SCRIPT_TASK_STATUS` (hash 0x74C14D31).
///
/// Forwards 3 script argument(s) to the engine: 3 integer(s).
/// No return slot is written.
export!(cdecl, rw_00bb8c60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
