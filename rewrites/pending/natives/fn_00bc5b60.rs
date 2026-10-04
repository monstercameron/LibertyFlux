// original: 0x00BC5B60 GET_CAR_LIVERY
use lf_k2_rt::{callee_cdecl, export, relocated};

/// GET_CAR_LIVERY: query a vehicle livery index.
///
/// Native handler. Forwards vehicle handle plus out-index word to the vehicle engine.
export!(cdecl, rw_00bc5b60(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        callee_cdecl!(1, u32, a0, a1)
    }
});
