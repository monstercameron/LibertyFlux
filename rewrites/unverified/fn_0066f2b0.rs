// original: 0x0066F2B0 tok_next_char (proposed)

/// Read the next input character, honouring pushed-back bytes.
///
/// `this` points to the tokenizer. When the pushback count at `+0x18` is
/// nonzero it is decremented and the byte at `+0x1C + count` is returned
/// sign-extended; otherwise the stream object at `+0x0C` supplies the byte,
/// either from its buffer (`+0x08`, position `+0x10` below end `+0x14`,
/// compared signed) or from virtual slot `+0x08` via the read callee, which
/// reports failure with anything but 1 (read as -1). A line feed increments
/// the line counter at `+0x08`. The returned value is the byte (or -1).
///
/// Original: 0x0066F2B0 (thiscall, no stack arguments, 1 call).
lf_checker_rt::export!(thiscall, rw_0066f2b0(this: u32) -> u32 {
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

        let count = rd32(this + COUNT);
        let c: u32;
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
            // The original passes the top byte of its pushed-ECX slot and
            // reads one byte back. A stub word write there would clobber the
            // return address, so the contract writes the word three bytes
            // lower (an aligned scratch slot on both sides) and both sides
            // take its top byte.
            let mut slot: u32 = 0;
            let byte_ptr = (slot_addr(&mut slot)).wrapping_add(3);
            let n = lf_checker_rt::callee_thiscall!(READ_CALLEE, u32, stream, byte_ptr, 1);
            if n != 1 {
                return 0xFFFF_FFFF;
            }
            c = slot >> 24;
        }
        }
        if c == LF {
            wr32(this + LINE, rd32(this + LINE).wrapping_add(1));
        }
        c
    }
});
