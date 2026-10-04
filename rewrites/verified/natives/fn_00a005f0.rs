// original: 0x00a005f0 ATTACH_OBJECT_TO_PED
/// Script native `ATTACH_OBJECT_TO_PED` (hash 0x577A699E).
///
/// Passes the call context straight through to a shared native unpacker
/// together with the engine worker's address: `unpack(engine_fn, ctx)`.
/// The address is an absolute reference in the original and is derived
/// with `relocated`, never hard-coded as a mapping.
export!(cdecl, rw_00a005f0(ctx: *const u8) -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0x00A02BD0), ctx as u32) }
});
