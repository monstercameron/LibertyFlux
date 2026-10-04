// original: 0x009cb9f0 CLOSE_MIC_PED
use lf_k2_rt::{callee_cdecl, export};
// CLOSE_MIC_PED: forward (ped, flag) to the engine call. No result.
export!(cdecl, rw_009CB9F0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, *a.add(1))
    }
});
