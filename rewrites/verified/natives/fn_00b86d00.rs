// original: 0x00b86d00 GET_VIEWPORT_POSITION_OF_COORD
/// Script native `GET_VIEWPORT_POSITION_OF_COORD` (hash 0x287A49A5).
///
/// Forwards six script arguments to the engine: three float
/// bit-patterns (coordinates) followed by three integers, and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b86d00(ctx: *const u8) -> u32 {
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
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
