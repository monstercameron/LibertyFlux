// original: 0x00a02040 SET_STATE_OF_CLOSEST_DOOR_OF_TYPE
/// Script native `SET_STATE_OF_CLOSEST_DOOR_OF_TYPE` (hash 0x10974B70).
///
/// Builds the engine argument block from a door-type id, four float coordinates, a boolean-coerced flag (whose word reuses the handler's own stack slot, see report) and one more float, then calls the engine worker. No return slot is written.
export!(cdecl, rw_00a02040(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let a3 = *args.add(3);
        let a4 = *args.add(4);
        let a5 = *args.add(5);
        let coerced = (ctx as u32 & !0xFF) | ((a4 != 0) as u32);
        callee_cdecl!(
            1,
            u32,
            a0,
            a1,
            a2,
            a3,
            coerced,
            a5,
            a1,
            a2,
            a3,
        )
    }
});
