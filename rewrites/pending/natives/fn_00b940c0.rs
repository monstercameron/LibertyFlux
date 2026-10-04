// original: 0x00b940c0 CLEAR_AREA_OF_COPS
/// Clear police presence around a point in the world.
///
/// Forwards the x/y/z coordinates and radius (script arguments 0-3, all
/// floats, copied bit-for-bit) to the engine implementation. Returns whatever
/// the engine call returned.
export!(cdecl, rw_00b940c0(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
