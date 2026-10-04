// original: 0x00bc5be0 GET_CAR_PITCH
/// Script native `GET_CAR_PITCH` (hash 0x61EE5C9A).
///
/// Forwards two script arguments (a vehicle handle and an output slot) to
/// the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00bc5be0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
