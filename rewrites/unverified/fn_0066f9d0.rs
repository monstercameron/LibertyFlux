// original: 0x0066F9D0 rage::fiTokenizer::vf5

/// Read the next token as an integer.
///
/// `this` points to the tokenizer and `required` says whether a value must
/// be present. A token is fetched into a 0x20-byte frame buffer. When the
/// fetch reports a token starting with `-`, or the token starts with a digit
/// (whichever the fetch reported), the integer-conversion callee parses the
/// buffer and its answer is returned. Otherwise the fallback is 0 when
/// required is nonzero and -1 when it is zero.
///
/// Original: 0x0066F9D0 (thiscall, one stack argument, 2 calls).
lf_checker_rt::export!(thiscall, rw_0066f9d0(this: u32, required: u32) -> u32 {
    unsafe {
        const TOKEN_SLOT: u32 = 0x08;
        const TOKEN_LEN: u32 = 0x20;
        const ATOI_CALLEE: u32 = 2;
        const MINUS: u8 = 0x2d;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read_unaligned() }
        }

        let vtable = rd32(this);
        let get_token: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + TOKEN_SLOT) as usize);
        let mut buf = [0u32; 8];
        let len = get_token(this, buf.as_mut_ptr() as u32, TOKEN_LEN);
        let bp = buf.as_mut_ptr() as u32;
        let first = rd8(bp);
        let mut parse = false;
        if len != 0 && first == MINUS {
            parse = true;
        } else {
            let d = first.wrapping_sub(0x30);
            if d <= 9 {
                parse = true;
            }
        }
        if parse {
            lf_checker_rt::callee_cdecl!(ATOI_CALLEE, u32, bp)
        } else if required != 0 {
            0
        } else {
            0xFFFF_FFFF
        }
    }
});
