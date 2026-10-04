// original: 0x00b9a800 GET_NEXT_CLOSEST_CAR_NODE_WITH_HEADING
/// Script native `GET_NEXT_CLOSEST_CAR_NODE_WITH_HEADING` (hash 0x3D7A673F).
///
/// Forwards seven script arguments to the engine node search: three
/// coordinate words copied as raw bits plus four integers. Stores the
/// low byte of the answer (a boolean) into the return slot.
export!(cdecl, rw_00b9a800(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            *args.add(6)
        );
        *slot = answer & 0xFF;
        slot as u32
    }
});
