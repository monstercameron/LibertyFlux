// original: 0x00bd8d70 NETWORK_SET_TALKER_FOCUS
use lf_k2_rt::{callee_cdecl, export};
/// Sets the network talker focus from one script argument.
///
/// Forwards the single argument word to the engine function.
export!(cdecl, rw_00BD8D70(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        callee_cdecl!(1, u32, *a)
    }
});
