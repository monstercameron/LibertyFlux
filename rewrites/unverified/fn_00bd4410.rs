// original: 0x00bd4410 START_PTFX_ON_OBJ_BONE
/// Script native `START_PTFX_ON_OBJ_BONE` (hash 0x60980323).
///
/// Forwards the call context together with a fixed engine address to
/// the engine worker. No return slot is written by the handler itself.
export!(cdecl, rw_00bd4410(ctx: *const u8) -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xBD60E0), ctx as u32) }
});
