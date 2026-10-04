// original: 0x00b86f10 IS_SCREEN_FADED_OUT
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `IS_SCREEN_FADED_OUT`: calls its engine function and writes the
/// low byte of the result (zero-extended) to the return slot.
export!(cdecl, rw_00b86f10(ctx: *const u32) -> u32 {
    unsafe {
        let slot = *ctx as *mut u32;
        let answer = callee_cdecl!(1, u32,);
        *slot = answer & 0xFF;
        0
    }
});
