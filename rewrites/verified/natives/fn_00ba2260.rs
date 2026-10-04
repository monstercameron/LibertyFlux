// original: 0x00ba2260 SET_NM_MESSAGE_INSTANCE_INDEX
/// Script native `SET_NM_MESSAGE_INSTANCE_INDEX` (hash 0x48543AED).
///
/// Forwards four script arguments (entity handles and a message index)
/// to the engine.
/// No return slot is written.
export!(cdecl, rw_00ba2260(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        );
        answer
    }
});
