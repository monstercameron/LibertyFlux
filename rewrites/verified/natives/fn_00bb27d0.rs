// original: 0x00BB27D0 PLAYER_HAS_GREYED_OUT_STARS
use lf_k2_rt::{callee_cdecl, export, relocated};

/// PLAYER_HAS_GREYED_OUT_STARS: test wanted-grey-out state.
///
/// Native handler. Forwards the player index, stores the boolean answer through the return slot.
export!(cdecl, rw_00bb27d0(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let ret_slot = *ctx_words as *mut u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let answer = callee_cdecl!(1, u32, a0);
        *ret_slot = answer & 0xFF;
        ret_slot as u32
    }
});
