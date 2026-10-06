// original: 0x00bf7320 fetch_copy_triple_b
/// Fetch through the loader, then copy this object's triple to the output.
///
/// Calls the cdecl loader with (`key`, this `+0x2c`) and ignores its answer,
/// then copies the three dwords at this `+0x30/0x34/0x38` to `out[0..3]`.
/// Returns the third word copied (this `+0x38`), not the loader's answer.
/// Twin of `fetch_copy_triple_a` with different offsets.
///
/// Original: 0x00BF7320 (thiscall, two stack words: key, out).
lf_checker_rt::export!(thiscall, rw_00bf7320(this: u32, key: u32, out: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const KEY_OFF: u32 = 0x2c;
        const TRIPLE: u32 = 0x30;
        lf_checker_rt::callee_cdecl!(1, u32, key, rd32(this + KEY_OFF));
        let x = rd32(this + TRIPLE);
        let y = rd32(this + TRIPLE + 4);
        let z = rd32(this + TRIPLE + 8);
        wr32(out, x);
        wr32(out + 4, y);
        wr32(out + 8, z);
        z
    }
});
