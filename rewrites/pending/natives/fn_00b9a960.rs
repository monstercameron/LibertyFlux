// original: 0x00b9a960 GET_NTH_CLOSEST_CAR_NODE_WITH_HEADING_ON_ISLAND
/// Script native `GET_NTH_CLOSEST_CAR_NODE_WITH_HEADING_ON_ISLAND` (hash 0x59DB1AD1).
///
/// Forwards ten script arguments to the engine: three float bit-patterns (a position) followed by seven integers (out-pointers and search keys), and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9a960(ctx: *const u8) -> u32 {
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
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
