// original: 0x0066F230 tok_pushback_token (proposed)

/// Push a token back onto the tokenizer so it is read again.
///
/// `this` points to the tokenizer, `text` to a NUL-terminated string and
/// `top` is the pushback index the token ends at. The pushback count at
/// `+0x18` becomes `top + 1`. Unless the text starts with a double quote,
/// the blank-search callee is asked for a space and then a tab in it: when
/// either is found the count grows by 2 more, a quote is stored ahead of the
/// token and `top` moves one up. The text is then copied backwards into the
/// buffer at `+0x1C` starting at `top`, so forward reads return it in order,
/// and the buffer head is stamped with a space (plus a quote when the token
/// needed quoting). The return register keeps only residue (its low byte is
/// always 0), so the contract does not compare it.
///
/// Original: 0x0066F230 (thiscall, two stack arguments, up to 2 calls).
lf_checker_rt::export!(thiscall, rw_0066f230(this: u32, text: u32, top: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x18;
        const PBUF: u32 = 0x1c;
        const FIND_CALLEE: u32 = 1;
        const QUOTE: u8 = 0x22;
        const SPACE: u8 = 0x20;

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
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write_unaligned(v) }
        }

        let mut top = top;
        wr32(this + COUNT, top.wrapping_add(1));
        let quoted: bool;
        if rd8(text) == QUOTE {
            quoted = true;
        } else {
            let sp = lf_checker_rt::callee_cdecl!(FIND_CALLEE, u32, text, SPACE as u32);
            let mut found = sp != 0;
            if !found {
                let tb = lf_checker_rt::callee_cdecl!(FIND_CALLEE, u32, text, 9);
                found = tb != 0;
            }
            if found {
                wr32(this + COUNT, rd32(this + COUNT).wrapping_add(2));
                wr8(top.wrapping_add(this).wrapping_add(PBUF + 2), QUOTE);
                top = top.wrapping_add(1);
                quoted = false;
            } else {
                quoted = true;
            }
        }
        let mut src = text;
        let mut dst = this.wrapping_add(PBUF).wrapping_add(top);
        let mut b = rd8(src);
        while b != 0 {
            src = src.wrapping_add(1);
            wr8(dst, b);
            b = rd8(src);
            dst = dst.wrapping_sub(1);
        }
        if quoted {
            wr8(this + PBUF, SPACE);
        } else {
            wr8(this + PBUF, SPACE);
            wr8(this + PBUF + 1, QUOTE);
        }
        0
    }
});
