// original: 0x00bd9490 SET_PED_EXISTS_ON_ALL_MACHINES
// Rewrite of the SET_PED_EXISTS_ON_ALL_MACHINES native handler.

/// Script native `SET_PED_EXISTS_ON_ALL_MACHINES(ped, exists)`.
///
/// Forwards the ped handle plus a boolean coercion of the flag argument. The
/// original builds the flag dword by setting the low byte of its own context
/// stack slot, so the high 24 bits repeat the context pointer; that exact
/// dword is reproduced here. No return slot is written; the engine's answer
/// is left in the return register.
export!(cdecl, rw_bd9490(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let flag = (ctx as u32 & 0xFFFFFF00) | ((*args.add(1) != 0) as u32);
        callee_cdecl!(1, u32, *args, flag)
    }
});
