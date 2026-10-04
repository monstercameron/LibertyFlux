// original: 0x00b9a910 GET_NTH_CLOSEST_CAR_NODE_WITH_HEADING
/// Script native `GET_NTH_CLOSEST_CAR_NODE_WITH_HEADING` (hash 0x1F6B3FF0).
///
/// Forwards eight script arguments to the engine: three float bit-patterns
/// (search position) followed by five integers (out-pointers and flags),
/// and stores the low byte of its answer (zero-extended) into the return
/// slot. Floats are copied as raw bits.
export!(cdecl, rw_00b9a910(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), *args.add(6), *args.add(7));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
