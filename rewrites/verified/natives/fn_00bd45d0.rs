// original: 0x00bd45d0 TRIGGER_PTFX_ON_PED_BONE
use lf_k2_rt::{callee_cdecl, export, relocated};
/// Triggers a particle effect on a ped bone via a shared dispatch helper.
///
/// Passes the engine effect routine's address together with this call's
/// own context pointer to the shared ptfx dispatch helper.
export!(cdecl, rw_00BD45D0(ctx: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0x00BD6720), ctx) }
});
