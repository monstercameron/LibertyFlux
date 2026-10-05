// original: 0x006701B0 rage::fiTokenizer::vf37

/// Write a string followed by `count` tabs, reporting full success.
///
/// `this` points to the tokenizer, `text` to a NUL-terminated string whose
/// first byte is nonzero in this proof (an empty string would select a
/// default through an unrelocated absolute address the checker cannot serve
/// on this machine) and `count` bounds the tab loop. The writer state at
/// `+0x14` becomes 0, the string (without its terminator) goes through the
/// string-write callee, then each tab goes either into the stream buffer or
/// through the tab-write callee, like the indent writer. Only the low byte of
/// the return value is meaningful: 1 when the string callee's answer plus
/// `count` equals the string length plus `count`, else 0. The original stages
/// the tab byte in its incoming-argument slot, so this proof runs with the
/// stack comparison off.
///
/// Original: 0x006701B0 (thiscall, two stack arguments, up to 1+N calls).
lf_checker_rt::export!(thiscall, rw_006701b0(this: u32, text: u32, count: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x14;
        const STREAM: u32 = 0x0c;
        const STR_WRITE_CALLEE: u32 = 1;
        const TAB_WRITE_CALLEE: u32 = 2;
        const TAB: u8 = 9;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn put(stream: u32, b: u8, wcallee: u32, hibase: u32) {
            unsafe {
                const STREAM_BUF: u32 = 0x08;
                const STREAM_POS: u32 = 0x10;
                const STREAM_MODE: u32 = 0x14;
                const STREAM_END: u32 = 0x18;
                let mode = ((stream + STREAM_MODE) as *const u32).read_unaligned();
                if mode != 0 {
                    slow(stream, b, wcallee, hibase);
                    return;
                }
                let pos = ((stream + STREAM_POS) as *const u32).read_unaligned();
                if (pos as i32) >= (((stream + STREAM_END) as *const u32).read_unaligned() as i32) {
                    slow(stream, b, wcallee, hibase);
                    return;
                }
                let buf = ((stream + STREAM_BUF) as *const u32).read_unaligned();
                ((buf + pos) as *mut u8).write_unaligned(b);
                ((stream + STREAM_POS) as *mut u32).write_unaligned(pos.wrapping_add(1));
            }
        }
        #[inline(always)]
        unsafe fn slow(stream: u32, b: u8, wcallee: u32, hibase: u32) {
            unsafe {
                // The original sets one byte of a word slot that still holds
                // an older value (`hibase`: its saved register or argument);
                // the cell reproduces that word exactly.
                let mut cell: u32 = (hibase & 0xFFFF_FF00) | (b as u32);
                lf_checker_rt::callee_thiscall!(wcallee, u32, stream, &mut cell as *mut u32 as u32, 1);
            }
        }
        ((this + STATE) as *mut u32).write_unaligned(0);
        debug_assert!(rd8(text) != 0);
        let mut len = 0u32;
        while rd8(text.wrapping_add(len)) != 0 {
            len = len.wrapping_add(1);
        }
        let stream = ((this + STREAM) as *const u32).read_unaligned();
        let wrote = lf_checker_rt::callee_thiscall!(STR_WRITE_CALLEE, u32, stream, text, len);
        let expect = len.wrapping_add(count);
        let got = wrote.wrapping_add(count);
        let mut left = count;
        while left != 0 {
            left = left.wrapping_sub(1);
            put(stream, TAB, TAB_WRITE_CALLEE, text);
        }
        (got == expect) as u32
    }
});
