// original: 0x0066FA20 rage::fiTokenizer::vf6

/// Read the next token as a double-precision float.
///
/// `this` points to the tokenizer and `required` (always nonzero in this
/// proof; see below) says whether a value must be present. A token is
/// fetched into a 0x20-byte frame buffer. When the fetch reports a token
/// starting with `-`, or the token starts with `.` or a digit, the
/// float-conversion callee parses the buffer and its double result is
/// returned. Otherwise the fallback is 0.0. (When required is zero the
/// original instead loads a constant through an unrelocated absolute address
/// the checker cannot serve on this machine, so that branch is not taken
/// here.)
///
/// Original: 0x0066FA20 (thiscall, one stack argument, 2 calls).
lf_checker_rt::export!(thiscall, rw_0066fa20(this: u32, required: u32) -> f64 {
    unsafe {
        const TOKEN_SLOT: u32 = 0x08;
        const TOKEN_LEN: u32 = 0x20;
        const ATOF_CALLEE: u32 = 2;
        const MINUS: u8 = 0x2d;
        const DOT: u8 = 0x2e;

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
        } else if first == DOT {
            parse = true;
        } else if first.wrapping_sub(0x30) <= 9 {
            parse = true;
        }
        if parse {
            lf_checker_rt::callee_cdecl!(ATOF_CALLEE, f64, bp)
        } else {
            debug_assert!(required != 0);
            0.0
        }
    }
});
