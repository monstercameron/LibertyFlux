// original: 0x009bb550 init_input_hook_copy (proposed)

/// Initialise an input-hook record from three source blocks, salting one
/// inherited word with a per-construction counter.
///
/// `this` points to the 0x5c-byte record. Word `+0x04` is adopted from the
/// caller and mixed with the low 14 bits of itself xored with the global
/// construction counter at 0x010327a0 (the counter then increments); the
/// record's vtable slot is set to the base table first and to the final
/// table once the header is complete. `stored` is kept at `+0x08`.
/// `by_ref` contributes the word it points to at `+0x0c`; `vec4` is four
/// words copied to `+0x10`..`+0x1c`; `wide` contributes twelve words to
/// `+0x20`..`+0x58`, skipping the words at `+0x0c`, `+0x1c` and `+0x2c` of
/// the source. Returns `this`.
///
/// Edge cases: none take another path; every pointer is dereferenced
/// unconditionally, so a bad one faults on both sides alike.
///
/// Original: thiscall, `this` in ECX, four stack words.
lf_checker_rt::export!(thiscall, rw_009bb550(this: u32, stored: u32, by_ref: u32, vec4: u32, wide: u32) -> u32 {
    unsafe {
        const BASE_VTABLE: u32 = 0x00e7e048;
        const FINAL_VTABLE: u32 = 0x00e948cc;
        const SALT_GLOBAL: u32 = 0x010327a0;
        const SALT_MASK: u32 = 0x3fff;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let salt = (lf_checker_rt::global::<u32>(SALT_GLOBAL) as *const u32).read_unaligned();
        let mixed = (rd(this + 0x04) ^ salt) & SALT_MASK;
        wr(this, lf_checker_rt::relocated(BASE_VTABLE));
        wr(this + 0x04, rd(this + 0x04) ^ mixed);
        (lf_checker_rt::global::<u32>(SALT_GLOBAL)).write_unaligned(salt.wrapping_add(1));
        wr(this + 0x08, stored);
        wr(this, lf_checker_rt::relocated(FINAL_VTABLE));
        wr(this + 0x0c, rd(by_ref));
        wr(this + 0x10, rd(vec4));
        wr(this + 0x14, rd(vec4 + 0x04));
        wr(this + 0x18, rd(vec4 + 0x08));
        wr(this + 0x1c, rd(vec4 + 0x0c));
        wr(this + 0x20, rd(wide));
        wr(this + 0x24, rd(wide + 0x04));
        wr(this + 0x28, rd(wide + 0x08));
        wr(this + 0x30, rd(wide + 0x10));
        wr(this + 0x34, rd(wide + 0x14));
        wr(this + 0x38, rd(wide + 0x18));
        wr(this + 0x40, rd(wide + 0x20));
        wr(this + 0x44, rd(wide + 0x24));
        wr(this + 0x48, rd(wide + 0x28));
        wr(this + 0x50, rd(wide + 0x30));
        wr(this + 0x54, rd(wide + 0x34));
        wr(this + 0x58, rd(wide + 0x38));
        this
    }
});
