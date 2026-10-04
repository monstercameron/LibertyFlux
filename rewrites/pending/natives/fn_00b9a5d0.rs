// original: 0x00b9a5d0 GET_CLOSEST_MAJOR_CAR_NODE
/// Script native `GET_CLOSEST_MAJOR_CAR_NODE` (hash 0x406A035E).
///
/// Forwards six script arguments to the engine: three float bit-patterns
/// (a position) and three integers (out-pointers). Stores the low byte of
/// the engine answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9a5d0(ctx: *const u8) -> u32 {
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
