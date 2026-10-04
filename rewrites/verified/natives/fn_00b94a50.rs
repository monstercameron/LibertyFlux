// original: 0x00b94a50 IS_PROJECTILE_IN_AREA
/// Script native `IS_PROJECTILE_IN_AREA` (hash 0x7BB35FCF).
///
/// Forwards six float script arguments (an area box) as raw bits to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b94a50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
