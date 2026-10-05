// original: 0x00670060 rage::fiTokenizer::vf28

/// Write an indented closing brace line and shallow the indent.
///
/// `this` points to the tokenizer. The level at `+0x220` is decremented
/// first, then the indent callee writes that many tabs, then `}`, a carriage
/// return and a line feed are each emitted either into the stream buffer or
/// through the write callee (same rule as the indent writer). The return
/// register is untouched, so the contract does not compare it.
///
/// Original: 0x00670060 (thiscall, no stack arguments, up to 4 calls).
lf_checker_rt::export!(thiscall, rw_00670060(this: u32) -> u32 {
    unsafe {
        const STREAM: u32 = 0x0c;
        const LEVEL: u32 = 0x220;
        const INDENT_CALLEE: u32 = 1;
        const WRITE_CALLEE: u32 = 2;

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
        let level = ((this + LEVEL) as *const u32).read_unaligned().wrapping_sub(1);
        ((this + LEVEL) as *mut u32).write_unaligned(level);
        lf_checker_rt::callee_thiscall!(INDENT_CALLEE, u32, this, level);
        let stream = ((this + STREAM) as *const u32).read_unaligned();
        put(stream, 0x7d, WRITE_CALLEE);
        put(stream, 0x0d, WRITE_CALLEE);
        put(stream, 0x0a, WRITE_CALLEE);
        0
    }
});
