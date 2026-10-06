// original: 0x00bf72f0 fetch_copy_triple_a
/// Fetch through the loader, then copy this object's triple to the output.
///
/// Calls the cdecl loader with (`key`, this `+0x24`) and ignores its answer,
/// then copies the three dwords at this `+0x18/0x1c/0x20` to `out[0..3]`.
/// Returns the third word copied (this `+0x20`), not the loader's answer.
/// Twin of `fetch_copy_triple_b` with different offsets.
///
/// Original: 0x00BF72F0 (thiscall, two stack words: key, out).
lf_checker_rt::export!(thiscall, rw_00bf72f0(this: u32, key: u32, out: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const KEY_OFF: u32 = 0x24;
        const TRIPLE: u32 = 0x18;
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
