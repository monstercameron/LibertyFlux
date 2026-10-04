// original: 0x00b9a290 ADD_SPAWN_BLOCKING_AREA
/// Script native `ADD_SPAWN_BLOCKING_AREA` (hash 0x36DF37DB).
///
/// Forwards four float script arguments to the engine as raw bits. The
/// call frame holds seven words: the four values followed by the first
/// three again, because the handler's float scratch area sits directly
/// above the pushed arguments. No return slot is written.
export!(cdecl, rw_00b9a290(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let a0 = *args;
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        callee_cdecl!(1, u32, a0, a1, a2, *args.add(3), a0, a1, a2)
    }
});
