// original: 0x00b9ece0 GET_CHAR_ANIM_IS_EVENT
/// Script native `GET_CHAR_ANIM_IS_EVENT`.
///
/// Forwards four script arguments to the engine and stores the low byte of
/// its answer (zero-extended) into the return slot. Returns the slot pointer.
export!(cdecl, rw_00b9ece0(ctx: *const u8) -> u32 {
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
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});

