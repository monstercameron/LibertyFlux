// original: 0x00BC73D0 SET_CAR_AS_MISSION_CAR
use lf_k2_rt::{callee_cdecl, export, relocated};

/// SET_CAR_AS_MISSION_CAR: flag a vehicle as mission-owned.
///
/// Native handler. Forwards the vehicle handle to the vehicle engine.
export!(cdecl, rw_00bc73d0(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        callee_cdecl!(1, u32, a0)
    }
});
