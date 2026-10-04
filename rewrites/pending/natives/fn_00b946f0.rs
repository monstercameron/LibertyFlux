// original: 0x00B946F0 GET_PROFILE_SETTING
use lf_k2_rt::{callee_cdecl, export, relocated};

/// GET_PROFILE_SETTING: read a profile setting value.
///
/// Native handler. Forwards the setting id to the profile engine, stores the full dword answer through the return slot.
export!(cdecl, rw_00b946f0(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let ret_slot = *ctx_words as *mut u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let answer = callee_cdecl!(1, u32, a0);
        *ret_slot = answer;
        answer
    }
});
