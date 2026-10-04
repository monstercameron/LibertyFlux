// original: 0x00b9fc80 IS_CHAR_ON_FOOT
use lf_k2_rt::{callee_cdecl, export};
/// Script native `IS_CHAR_ON_FOOT`.
/// Passes the character handle to the ped engine function and writes
/// its low result byte to the script return slot.
export!(cdecl, rw_00b9fc80(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let ret_slot = *ctx_words as *mut u32;
        let answer: u32 = callee_cdecl!(1, u32, a0);
        // The handler keeps only the low byte (movzx).
        *ret_slot = answer & 0xFF;
        ret_slot as u32
    }
});
