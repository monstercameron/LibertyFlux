// original: 0x00B951F0 STRING_DIFFERENCE
/// Compares two strings, storing the engine result in the return slot.
///
/// Forwards both string words to the engine compare function and stores
/// its full 32-bit answer in the return slot.
lf_rn26_rt::export!(cdecl, rw_00B951F0(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        let ans: u32 = lf_rn26_rt::callee_cdecl!(1, u32, *a, *a.add(1));
        let ret = *(ctx as *const *mut u32);
        *ret = ans;
        ans
    }
});
