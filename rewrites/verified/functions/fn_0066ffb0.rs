// original: 0x0066FFB0 rage::fiTokenizer::vf27

/// Write an indented opening brace line and deepen the indent.
///
/// `this` points to the tokenizer. The indent callee first writes as many
/// tabs as the level at `+0x220`, then `{`, a carriage return and a line feed
/// are each emitted either into the stream buffer or through the write
/// callee (same rule as the indent writer), and the level is incremented.
/// The one-byte cell handed to the write callee shares its word slot with
/// the saved `this` pointer, whose high bytes the rewrite reproduces exactly.
/// The return register is untouched, so the contract does not compare it.
///
/// Original: 0x0066FFB0 (thiscall, no stack arguments, up to 4 calls).
lf_checker_rt::export!(thiscall, rw_0066ffb0(this: u32) -> u32 {
    unsafe {
        const STREAM: u32 = 0x0c;
        const LEVEL: u32 = 0x220;
        const INDENT_CALLEE: u32 = 1;
        const WRITE_CALLEE: u32 = 2;

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
        let level = ((this + LEVEL) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(INDENT_CALLEE, u32, this, level);
        let stream = ((this + STREAM) as *const u32).read_unaligned();
        put(stream, 0x7b, WRITE_CALLEE, this);
        put(stream, 0x0d, WRITE_CALLEE, this);
        put(stream, 0x0a, WRITE_CALLEE, this);
        ((this + LEVEL) as *mut u32).write_unaligned(level.wrapping_add(1));
        0
    }
});
