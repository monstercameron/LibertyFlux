// original: 0x00BD8BD0 NETWORK_IS_LINK_CONNECTED
/// Reports whether the network link is connected (0 or 1 in return slot).
///
/// Calls the engine query with no arguments, zero-extends its low byte
/// and stores that in the return slot. Returns the return-slot pointer
/// (the original reloads it into `eax` for the store).
lf_rn26_rt::export!(cdecl, rw_00BD8BD0(ctx: *const u8) -> u32 {
    unsafe {
        let ans: u32 = lf_rn26_rt::callee_cdecl!(1, u32,);
        let ret = *(ctx as *const *mut u32);
        *ret = ans & 0xFF;
        ret as u32
    }
});
