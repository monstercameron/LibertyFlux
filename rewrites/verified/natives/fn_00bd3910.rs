// original: 0x00bd3910 CREATE_USER_3D_MARKER
/// Script native `CREATE_USER_3D_MARKER` (hash 0x77513211).
///
/// Forwards four script arguments to the engine: three float bit-patterns
/// (marker position) and one integer. Stores the engine's full 32-bit
/// answer into the return slot.
export!(cdecl, rw_00bd3910(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3),);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
