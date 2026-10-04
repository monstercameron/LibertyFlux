// original: 0x009e4e10 ped_bit_fill (proposed)

/// Set one bit of a three-word bitmap and fill through it.
///
/// Builds a 12-byte bitmap in its frame with bit `a2` set
/// (`1 << (a2 & 31)` in word `a2 >> 5`; `a2` is constrained to 0..95
/// by the contract, since larger indexes would spill into the
/// original's frame, noted in `narrowed`) and calls fill (`thiscall`
/// on `this + 0xb34` with `(a1, word0, word1, word2)`; the pushed `a1`
/// slot doubles as the bitmap's first word). Then runs the CRT
/// security-cookie check, whose xor'd cookie in `ecx` differs
/// legitimately between the two frames, so the contract compares no
/// registers for that call. Returns fill's answer. `thiscall`, two
/// stack words.
lf_checker_rt::export!(thiscall, rw_009e4e10(this: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const SET_OFF: u32 = 0xb34;
        const FILL: u32 = 1;
        const COOKIE_CHECK: u32 = 2;
        let word = ((a2 as i32) >> 5) as u32;
        let bit = a2 & 0x1f;
        let mut bm = [0u32; 3];
        bm[word as usize] |= 1u32 << bit;
        let set = this.wrapping_add(SET_OFF);
        let r = lf_checker_rt::callee_thiscall!(FILL, u32, set, a1, bm[0], bm[1], bm[2]);
        // Zero-argument call through the stub table (the callee macro
        // needs at least one argument); the stub preserves registers.
        let chk: extern "cdecl" fn() -> u32 =
            unsafe { core::mem::transmute(lf_checker_rt::callee_addr(COOKIE_CHECK) as usize) };
        let _ = chk();
        r
    }
});
