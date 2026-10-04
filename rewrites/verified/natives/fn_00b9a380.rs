// original: 0x00b9a380 CALCULATE_TRAVEL_DISTANCE_BETWEEN_NODES
/// Script native `CALCULATE_TRAVEL_DISTANCE_BETWEEN_NODES` (hash 0x09A558A5).
///
/// Forwards six script words (two 3D node positions as float bit patterns)
/// to the engine, which answers with a float distance in ST0, and stores
/// those exact bits into the return slot.
export!(cdecl, rw_00b9a380(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer: f32 = callee_cdecl!(
            1,
            f32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5)
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer.to_bits();
        slot as u32
    }
});
