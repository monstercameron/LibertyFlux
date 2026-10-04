// original: 0x00a00cd0 GET_LEVEL_DESIGN_COORDS_FOR_OBJECT
/// Script native `GET_LEVEL_DESIGN_COORDS_FOR_OBJECT` (hash 0x3E762D9D).
///
/// Forwards five script arguments (an object handle plus out-pointer slots
/// for the coordinates) to the engine. No return slot is written.
export!(cdecl, rw_00a00cd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4)
        )
    }
});
