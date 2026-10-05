// original: 0x0066F660 tok_match_and_pushback (proposed)

/// Fetch a token, compare it inline, and push it back unless consumed.
///
/// `this` points to the tokenizer, `expected` to the expected string and
/// only the low byte of `flag` matters. A token is fetched into a 0x200-byte
/// frame buffer. When the fetch reports none, or the token differs from the
/// expected string, the token is pushed back and 0 is returned. When they
/// match, the token is pushed back only if the flag byte is zero, and 1 is
/// returned either way. Only the low byte of the return value is meaningful;
/// the current character at `+0x10` is preserved across the call.
///
/// Original: 0x0066F660 (thiscall, two stack arguments, 2 calls).
lf_checker_rt::export!(thiscall, rw_0066f660(this: u32, expected: u32, flag: u32) -> u32 {
    unsafe {
        const CUR: u32 = 0x10;
        const TOKEN_SLOT: u32 = 0x08;
        const TOKEN_LEN: u32 = 0x200;
        const PUSHBACK_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read_unaligned() }
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
        let bp = buf.as_mut_ptr() as u32;
        let mut i = 0u32;
        let mut diff = false;
        loop {
            let a = rd8(expected.wrapping_add(i));
            let b = rd8(bp.wrapping_add(i));
            if a != b {
                diff = true;
                break;
            }
            if a == 0 {
                break;
            }
            let a1 = rd8(expected.wrapping_add(i).wrapping_add(1));
            let b1 = rd8(bp.wrapping_add(i).wrapping_add(1));
            if a1 != b1 {
                diff = true;
                break;
            }
            i = i.wrapping_add(2);
            if a1 == 0 {
                break;
            }
        }
        if diff {
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
