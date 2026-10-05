// original: 0x0066F710 tok_match_and_pushback_named (proposed)

/// Fetch a token, compare it through the string callee, push it back unless
/// consumed.
///
/// `this` points to the tokenizer, `expected` to the expected string and
/// only the low byte of `flag` matters. A token is fetched into a 0x200-byte
/// frame buffer. When the fetch reports none, or the string callee reports a
/// difference, the token is pushed back and 0 is returned. On a match the
/// token is pushed back only if the flag byte is zero, and 1 is returned
/// either way. Only the low byte of the return value is meaningful; the
/// current character at `+0x10` is preserved across the call.
///
/// Original: 0x0066F710 (thiscall, two stack arguments, 3 calls).
lf_checker_rt::export!(thiscall, rw_0066f710(this: u32, expected: u32, flag: u32) -> u32 {
    unsafe {
        const CUR: u32 = 0x10;
        const TOKEN_SLOT: u32 = 0x08;
        const TOKEN_LEN: u32 = 0x200;
        const CMP_CALLEE: u32 = 2;
        const PUSHBACK_CALLEE: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let vtable = rd32(this);
        let get_token: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + TOKEN_SLOT) as usize);
        let saved = rd32(this + CUR);
        let mut buf = [0u32; 128];
        let len = get_token(this, buf.as_mut_ptr() as u32, TOKEN_LEN);
        if len == 0 {
            lf_checker_rt::callee_thiscall!(PUSHBACK_CALLEE, u32, this, buf.as_mut_ptr() as u32, len);
            wr32(this + CUR, saved);
            return 0;
        }
        let d = lf_checker_rt::callee_cdecl!(CMP_CALLEE, u32, expected, buf.as_mut_ptr() as u32);
        if d != 0 {
            lf_checker_rt::callee_thiscall!(PUSHBACK_CALLEE, u32, this, buf.as_mut_ptr() as u32, len);
            wr32(this + CUR, saved);
            return 0;
        }
        if (flag & 0xFF) == 0 {
            lf_checker_rt::callee_thiscall!(PUSHBACK_CALLEE, u32, this, buf.as_mut_ptr() as u32, len);
            wr32(this + CUR, saved);
        }
        1
    }
});
