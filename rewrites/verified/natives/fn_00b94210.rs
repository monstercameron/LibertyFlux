// original: 0x00b94210 FIRE_SINGLE_BULLET
/// Fire one bullet from a point towards a target.
///
/// Forwards the from/to coordinates (script arguments 0-5, floats) and the
/// weapon id (argument 6) to the engine implementation. Returns whatever the
/// engine call returned.
export!(cdecl, rw_00b94210(ctx: u32) -> u32 {
    unsafe {
        let a = *(ctx as *const u32).add(2) as *const u32;
        callee_cdecl!(
            1, u32, *a, *a.add(1), *a.add(2), *a.add(3), *a.add(4), *a.add(5), *a.add(6)
        )
    }
});
