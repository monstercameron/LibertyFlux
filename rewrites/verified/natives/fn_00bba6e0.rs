// original: 0x00bba6e0 TASK_SHIMMY_IN_DIRECTION
/// Script native `TASK_SHIMMY_IN_DIRECTION` (hash 0x7B1A5333).
///
/// Forwards 2 script arguments to the engine in order.
/// Stores the low byte of the engine answer (zero-extended)
/// into the return slot.
export!(cdecl, rw_00bba6e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
