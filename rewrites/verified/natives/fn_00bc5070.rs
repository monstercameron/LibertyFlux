// original: 0x00bc5070 APPLY_FORCE_TO_CAR
/// Script native `APPLY_FORCE_TO_CAR` (hash 0x434611A3).
///
/// Takes twelve script arguments (a vehicle handle plus force and offset vectors). The handler forwards its call context together with an engine-function id to a shared unpacker routine (intercepted and scripted by the checker), which reads the twelve arguments itself. No return slot is written.
export!(cdecl, rw_00bc5070(ctx: *const u8) -> u32 {
    unsafe {
        // Engine-function id consumed by the shared unpacker (arg0); the
        // call context is arg1. The id is a relocated immediate.
        const ENGINE_ID: u32 = 0x00BC8E40;
        callee_cdecl!(1, u32, relocated(ENGINE_ID), ctx as u32)
    }
});
