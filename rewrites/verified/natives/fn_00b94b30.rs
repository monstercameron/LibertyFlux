// original: 0x00b94b30 IS_SNIPER_INVERTED
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `IS_SNIPER_INVERTED`: calls its engine function and writes the
/// low byte of the result (zero-extended) to the return slot.
export!(cdecl, rw_00b94b30(ctx: *const u32) -> u32 {
    unsafe {
        let slot = *ctx as *mut u32;
        let answer = callee_cdecl!(1, u32,);
        *slot = answer & 0xFF;
        0
    }
});
