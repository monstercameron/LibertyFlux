// original: 0x00B9FC00 IS_CHAR_IN_ZONE
use lf_k2_rt::{callee_cdecl, export, relocated};

/// IS_CHAR_IN_ZONE: test whether a ped is inside a named zone.
///
/// Native handler. Forwards ped handle and zone name, stores the boolean answer through the return slot.
export!(cdecl, rw_00b9fc00(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let ret_slot = *ctx_words as *mut u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let answer = callee_cdecl!(1, u32, a0, a1);
        *ret_slot = answer & 0xFF;
        ret_slot as u32
    }
});
