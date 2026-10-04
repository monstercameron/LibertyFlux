// original: 0x00a005d0 ATTACH_OBJECT_TO_OBJECT_PHYSICALLY
/// ATTACH_OBJECT_TO_OBJECT_PHYSICALLY: Attaches one object to another physically.
/// Passes the call context plus the engine routine address to the
/// shared native unpacker, which reads the script arguments itself.
lf_rn109_rt::export!(cdecl, rw_fn_00a005d0(ctx: u32) -> u32 {
    let routine = lf_rn109_rt::relocated(0x00a02aa0);
    lf_rn109_rt::callee_cdecl!(1, u32, routine, ctx,)
});
