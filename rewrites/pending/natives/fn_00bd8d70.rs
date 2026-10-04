// original: 0x00BD8D70 NETWORK_SET_TALKER_FOCUS
/// Sets the network talker focus from one script argument.
///
/// Forwards the single argument word to the engine function.
lf_rn26_rt::export!(cdecl, rw_00BD8D70(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        lf_rn26_rt::callee_cdecl!(1, u32, *a)
    }
});
