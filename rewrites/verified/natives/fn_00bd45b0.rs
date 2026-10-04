// original: 0x00bd45b0 TRIGGER_PTFX_ON_PED
// rw_trigger_ptfx_on_ped: native TRIGGER_PTFX_ON_PED (handler 0x00BD45B0).
//
// Passes the call context plus the per-type particle-effect implementation address to the shared 9-arg unpacker.
export!(cdecl, rw_trigger_ptfx_on_ped(ctx: *mut u32) -> u32 {
    unsafe {
        let engine_fn = relocated(0x00BD6670);
        callee_cdecl!(1, u32, engine_fn, ctx as u32)
    }
});
