// original: 0x00BD45D0 TRIGGER_PTFX_ON_PED_BONE
/// Triggers a particle effect on a ped bone via a shared dispatch helper.
///
/// Passes the engine effect routine's address together with this call's
/// own context pointer to the shared ptfx dispatch helper.
lf_rn26_rt::export!(cdecl, rw_00BD45D0(ctx: u32) -> u32 {
    unsafe { lf_rn26_rt::callee_cdecl!(1, u32, lf_rn26_rt::relocated(0x00BD6720), ctx) }
});
