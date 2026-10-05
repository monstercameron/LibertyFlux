// original: 0x00b0a910 net_record_copy (proposed)

/// Copy one network record from `src` into `this`, skipping the scratch
/// words and merging one flag byte.
///
/// `this` (ecx) and `src` (the one stack word) each point to a 0x4c-byte
/// record. Nine dwords (`+0x00..+0x20`) are copied, the three words at
/// `+0x24..+0x2c` are left alone, then one dword (`+0x30`), two float slots
/// (`+0x34`, `+0x38`, moved as bits), one dword (`+0x3c`), two half-words
/// (`+0x40`, `+0x42`), one byte (`+0x44`) and one byte (`+0x45`) are copied,
/// the low three bits of the flag byte at `+0x46` are taken from the source
/// while its high five bits are kept, and the last dword (`+0x48`) is copied.
/// The original merges `+0x45` one bit at a time; over all 65,536
/// source/destination pairs the sequence equals a plain copy (checked by
/// exhaustive simulation), and the `+0x46` sequence equals
/// `(dst & 0xf8) | (src & 0x07)` likewise.
///
/// Returns `this`. No calls, no globals, no arithmetic. thiscall: the
/// destination arrives in ecx, the source is the single stack word, and the
/// callee pops it (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00b0a910(this: u32, src: u32) -> u32 {
    unsafe {
        const HEAD_WORDS: usize = 9;
        const GAP_WORDS: u32 = 3;
        const TAIL_DWORD: u32 = 0x30;
        const FLOAT0: u32 = 0x34;
        const FLOAT1: u32 = 0x38;
        const AFTER_FLOATS: u32 = 0x3c;
        const HALF0: u32 = 0x40;
        const HALF1: u32 = 0x42;
        const FLAG_COPY: u32 = 0x44;
        const FLAG_MERGED_BITWISE: u32 = 0x45;
        const FLAG_MERGED_LOW3: u32 = 0x46;
        const FLAG_LOW3_MASK: u8 = 0x07;
        const LAST_DWORD: u32 = 0x48;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let dst = this;
        for i in 0..HEAD_WORDS {
            wr32(dst + (i as u32) * 4, rd32(src + (i as u32) * 4));
        }
        // Words dst+0x24..+0x2c (GAP_WORDS of them) are deliberately untouched.
        let _ = GAP_WORDS;
        wr32(dst + TAIL_DWORD, rd32(src + TAIL_DWORD));
        let f0 = rd32(src + FLOAT0);
        let f1 = rd32(src + FLOAT1);
        wr32(dst + FLOAT0, f0);
        wr32(dst + FLOAT1, f1);
        wr32(dst + AFTER_FLOATS, rd32(src + AFTER_FLOATS));
        wr16(dst + HALF0, rd16(src + HALF0));
        wr16(dst + HALF1, rd16(src + HALF1));
        wr8(dst + FLAG_COPY, rd8(src + FLAG_COPY));
        wr8(dst + FLAG_MERGED_BITWISE, rd8(src + FLAG_MERGED_BITWISE));
        let kept = rd8(dst + FLAG_MERGED_LOW3) & !FLAG_LOW3_MASK;
        let taken = rd8(src + FLAG_MERGED_LOW3) & FLAG_LOW3_MASK;
        wr8(dst + FLAG_MERGED_LOW3, kept | taken);
        wr32(dst + LAST_DWORD, rd32(src + LAST_DWORD));
        dst
    }
});
