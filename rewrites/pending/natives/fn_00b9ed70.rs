// original: 0x00b9ed70 GET_CHAR_COORDINATES
/// GET_CHAR_COORDINATES: Returns the character's coordinates; forward 4 script arguments to the engine implementation.
lf_rn109_rt::export!(cdecl, rw_fn_00b9ed70(ctx: u32) -> u32 {
    let args = unsafe { *((ctx + 8) as *const u32) };
    let a0 = unsafe { *((args + 0) as *const u32) };
    let a1 = unsafe { *((args + 4) as *const u32) };
    let a2 = unsafe { *((args + 8) as *const u32) };
    let a3 = unsafe { *((args + 12) as *const u32) };
    lf_rn109_rt::callee_cdecl!(1, u32, a0, a1, a2, a3,)
});
