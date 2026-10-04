// original: 0x00b9a790 GET_NEXT_CLOSEST_CAR_NODE
/// Script native `GET_NEXT_CLOSEST_CAR_NODE` (hash 0x5935382A).
///
/// Forwards six script arguments to the engine: three float
/// bit-patterns (coordinates) followed by three integers, and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9a790(ctx: *const u8) -> u32 {
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
