// original: 0x00bc6120 GET_PLANE_UNDERCARRIAGE_POSITION
/// Script native `GET_PLANE_UNDERCARRIAGE_POSITION` (hash 0x353F0568).
///
/// Forwards a vehicle handle and an out-pointer to the engine.
/// No return slot is written.
export!(cdecl, rw_00bc6120(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1),)
    }
});
