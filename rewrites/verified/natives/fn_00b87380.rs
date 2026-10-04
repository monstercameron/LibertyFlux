// original: 0x00b87380 SET_CAM_FOV
/// Script native `SET_CAM_FOV` (hash 0x55D470C2).
///
/// Forwards two script arguments (a camera handle and a float field of
/// view) to the engine. The float travels through an SSE register in the
/// original, but only its bit pattern is copied, so the rewrite forwards
/// it as an integer word: bit-exact by construction.
export!(cdecl, rw_00b87380(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
