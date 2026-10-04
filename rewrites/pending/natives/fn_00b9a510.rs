// original: 0x00b9a510 GET_CLOSEST_CAR_NODE
/// Script native `GET_CLOSEST_CAR_NODE` (hash 0x27F87222).
///
/// Forwards six script arguments to the engine: three float bit-patterns
/// (a position) and three integers (search parameters). The floats travel
/// as raw bits and are bit-exact by construction. Stores the low byte of
/// the engine answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9a510(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer: u32 = callee_cdecl!(
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
