// original: 0x00b8ce10 IS_THIS_PRINT_BEING_DISPLAYED
/// Script native `IS_THIS_PRINT_BEING_DISPLAYED` (hash 0x459A7F23).
///
/// Forwards eleven script arguments to the engine and stores the low byte
/// of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b8ce10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(
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
            *args.add(8),
            *args.add(9),
            *args.add(10),
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
