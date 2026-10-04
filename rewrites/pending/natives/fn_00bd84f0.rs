// original: 0x00BD84F0 NETWORK_GET_NUM_UNFILLED_RESERVATIONS
use lf_k2_rt::{callee_cdecl, export, relocated};

/// NETWORK_GET_NUM_UNFILLED_RESERVATIONS: count open net slots.
///
/// Native handler. Takes no script args; calls the network engine and stores the count through the return slot.
export!(cdecl, rw_00bd84f0(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let ret_slot = *ctx_words as *mut u32;
        let answer = callee_cdecl!(1, u32,);
        *ret_slot = answer;
        answer
    }
});
