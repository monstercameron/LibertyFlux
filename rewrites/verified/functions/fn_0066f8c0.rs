// original: 0x0066F8C0 tok_skip_to_eol (proposed)

/// Consume input up to and including the next line feed.
///
/// `this` points to the tokenizer. Characters are consumed with the same
/// reader as the next-character routine (pushback count `+0x18` and buffer
/// `+0x1C`, read sign-extended, else the stream at `+0x0C`, else the read
/// callee): a line feed increments the line counter at `+0x08` and ends the
/// scan, -1 (a 0xFF pushback byte) ends it too, and a read failure returns
/// the callee's answer straight away. The returned value is the terminating
/// character (normally a line feed).
///
/// Original: 0x0066F8C0 (thiscall, no stack arguments, 1 call).
lf_checker_rt::export!(thiscall, rw_0066f8c0(this: u32) -> u32 {
    unsafe {
        const LINE: u32 = 0x08;
        const STREAM: u32 = 0x0c;
        const COUNT: u32 = 0x18;
        const PBUF: u32 = 0x1c;
        const S_BUF: u32 = 0x08;
        const S_POS: u32 = 0x10;
        const S_END: u32 = 0x14;
        const READ_CALLEE: u32 = 1;
        const LF: u32 = 0x0a;
        const EOF: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u32 {
            unsafe { (a as *const u8).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn slot_addr(s: &mut u32) -> u32 {
            unsafe { (s as *mut u32) as u32 }
        }

        loop {
            let c: u32;
            let count = rd32(this + COUNT);
            if count != 0 {
                let top = count.wrapping_sub(1);
                wr32(this + COUNT, top);
                c = ((this + PBUF + top) as *const i8).read_unaligned() as i32 as u32;
            } else {
                let stream = rd32(this + STREAM);
                let pos = rd32(stream + S_POS);
                if (pos as i32) < (rd32(stream + S_END) as i32) {
                    let buf = rd32(stream + S_BUF);
                    c = rd8(buf + pos);
                    wr32(stream + S_POS, pos.wrapping_add(1));
                } else {
                    let mut slot: u32 = 0;
                    let byte_ptr = (slot_addr(&mut slot)).wrapping_add(3);
                    let n = lf_checker_rt::callee_thiscall!(READ_CALLEE, u32, stream, byte_ptr, 1);
                    if n != 1 {
                        return n;
                    }
                    c = slot >> 24;
                }
            }
            if c == LF {
                wr32(this + LINE, rd32(this + LINE).wrapping_add(1));
                return c;
            }
            if c == EOF {
                return c;
            }
        }
    }
});
