// original: 0x00670160 tok_write_indent (proposed)

/// Write `count` tab characters to the tokenizer's stream.
///
/// `this` points to the tokenizer. Each tab goes either straight into the
/// stream buffer (stream object at `+0x0C`, buffer `+0x08`, position `+0x10`
/// below end `+0x18`, compared signed, when the mode at `+0x14` is zero) or
/// through the write callee with a one-byte frame cell. The original stages
/// that byte in its own incoming-argument slot, so this proof runs with the
/// stack comparison off. The return register is untouched, so the contract
/// does not compare it.
///
/// Original: 0x00670160 (thiscall, one stack argument, up to N calls).
lf_checker_rt::export!(thiscall, rw_00670160(this: u32, count: u32) -> u32 {
    unsafe {
        const STREAM: u32 = 0x0c;
        const WRITE_CALLEE: u32 = 1;
        const TAB: u8 = 9;

        #[inline(always)]
        unsafe fn put(stream: u32, b: u8, wcallee: u32) {
            unsafe {
                const STREAM_BUF: u32 = 0x08;
                const STREAM_POS: u32 = 0x10;
                const STREAM_MODE: u32 = 0x14;
                const STREAM_END: u32 = 0x18;
                let mode = ((stream + STREAM_MODE) as *const u32).read_unaligned();
                if mode != 0 {
                    slow(stream, b, wcallee);
                    return;
                }
                let pos = ((stream + STREAM_POS) as *const u32).read_unaligned();
                if (pos as i32) >= (((stream + STREAM_END) as *const u32).read_unaligned() as i32) {
                    slow(stream, b, wcallee);
                    return;
                }
                let buf = ((stream + STREAM_BUF) as *const u32).read_unaligned();
                ((buf + pos) as *mut u8).write_unaligned(b);
                ((stream + STREAM_POS) as *mut u32).write_unaligned(pos.wrapping_add(1));
            }
        }
        #[inline(always)]
        unsafe fn slow(stream: u32, b: u8, wcallee: u32) {
            unsafe {
                let mut cell = [0u8; 4];
                cell[0] = b;
                lf_checker_rt::callee_thiscall!(wcallee, u32, stream, cell.as_mut_ptr() as u32, 1);
            }
        }
        let stream = ((this + STREAM) as *const u32).read_unaligned();
        let mut left = count;
        while left != 0 {
            left = left.wrapping_sub(1);
            put(stream, TAB, WRITE_CALLEE);
        }
        0
    }
});
