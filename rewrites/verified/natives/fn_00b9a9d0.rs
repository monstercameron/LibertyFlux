// original: 0x00b9a9d0 GET_PARKING_NODE_IN_AREA
/// Script native `GET_PARKING_NODE_IN_AREA` (hash 0x70CB4DCE).
///
/// Finds a parking node in an area: forwards six float bit-patterns
/// (area bounds) and three more words to the engine. No return slot
/// is written.
export!(cdecl, rw_00b9a9d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
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
        )
    }
});
