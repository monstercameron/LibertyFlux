// original: 0x00698C70 bitstream_read_bits (proposed)

/// Gathers `count` bits out of a bit array through an auto-increment cursor.
///
/// `obj` points at a header whose word at `+0` is the bit-array base.
/// `cursor` points at a word holding the starting bit index; it is
/// incremented once per gathered bit. Bit `c` lives in word `c >> 5` at
/// position `c & 31`; gathered bits pack into the result low bit first.
/// A zero count gathers nothing and leaves the cursor alone. The original
/// also spills the bit-array base into its incoming second argument slot
/// as scratch; that store is not reproduced (its value is observed through
/// the gathered bits in the return value).
///
/// Original: thiscall `(obj: ecx, cursor, count) -> eax`, no calls.
lf_checker_rt::export!(thiscall, rw_00698C70(obj: u32, cursor: u32, count: u32) -> u32 {
    unsafe {
        const BITS_PTR: u32 = 0;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        if count == 0 {
            return 0;
        }
        let bits = rd32(obj.wrapping_add(BITS_PTR));
        let mut out: u32 = 0;
        let mut mask: u32 = 1;
        let mut left = count;
        while left != 0 {
            let c = rd32(cursor);
            wr32(cursor, c.wrapping_add(1));
            let word = rd32(bits.wrapping_add((c >> 5).wrapping_mul(4)));
            if (word.wrapping_shr(c & 31) & 1) != 0 {
                out |= mask;
            }
            mask = mask.wrapping_add(mask);
            left = left.wrapping_sub(1);
        }
        out
    }
});
