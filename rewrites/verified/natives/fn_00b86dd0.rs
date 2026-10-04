// original: 0x00b86dd0 IS_CAM_COLLIDING
/// Script native `IS_CAM_COLLIDING` (hash 0x39595CE1).
///
/// Forwards two script arguments (camera handles) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b86dd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
