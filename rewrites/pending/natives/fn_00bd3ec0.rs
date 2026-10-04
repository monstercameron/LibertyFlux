// original: 0x00bd3ec0 ENABLE_DEFERRED_LIGHTING
use lf_k2_rt::{callee_cdecl, export};
/// Toggle deferred lighting: coerce the script argument to 0/1 and pass
/// it to the engine. The original coerces through a stack temporary that
/// leaves the context pointer's high bytes in the pushed dword; those
/// residue bytes are masked in the contract (call_skip) and only the low
/// byte is significant.
export!(cdecl, rw_00bd3ec0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let flag = u32::from(*args != 0);
        callee_cdecl!(1, u32, flag);
        0
    }
});
