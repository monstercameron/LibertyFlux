// original: 0x00b86710 CAM_IS_SPHERE_VISIBLE
/// Script native `CAM_IS_SPHERE_VISIBLE` (hash 0x2D5611D4).
///
/// Forwards a camera handle and four float bit-patterns (sphere centre and
/// radius) to the engine, then stores the low byte of its answer
/// (zero-extended) into the return slot.
export!(cdecl, rw_00b86710(ctx: *const u8) -> u32 {
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
        );
        *slot = answer & 0xFF;
        slot as u32
    }
});
