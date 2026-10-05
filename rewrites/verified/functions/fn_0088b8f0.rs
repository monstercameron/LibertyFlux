// original: 0x0088b8f0 rage::audVoiceDSound::vf8
/// Refresh one voice slot's sample window and advance its ring cursor.
///
/// `this` points to the voice object. The dword at `+0xe0` selects one of
/// several 64-byte slots; the pointer at `+0x14` is the current sample
/// source. The function copies 14 dwords from the source into this slot's
/// window at `+0xec`, optionally refills the cached voice pointer at
/// `+0xc8` through two helper calls when the incoming sample count is
/// non-zero and flag bit `0x10` at `+0x8c` is set, then derives two
/// counters from the slot: the low dword of `floor(a * b * 0.001)` doubled
/// (where `a` is the dword at `+0xc` and `b` the surviving sample count),
/// stored at slot offset `+0x124`, and the source word at `+0xc` minus that
/// value at `+0x128`. Finally the cursor becomes `(cursor + 1) % divisor`
/// with the divisor at `+0xe8`, and the quotient is returned.
///
/// The original computes the float-to-int step with an SSE round-to-integer
/// idiom followed by `fistp` under a truncate control word; that sequence
/// is exactly `floor`, and the stored value is its low dword (`0` when the
/// value is not finite or outside the 64-bit range). Both helpers are
/// intercepted by the checker: the first is cdecl with two arguments, the
/// second cleans its single argument itself.
///
/// Original: 0x0088b8f0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0088b8f0(this: u32, count: u32) -> u32 {
    unsafe {
        const SLOT_CURSOR: u32 = 0xe0;
        const SLOT_DIVISOR: u32 = 0xe8;
        const SAMPLE_SRC: u32 = 0x14;
        const SLOT_WINDOW: u32 = 0xec;
        const WINDOW_WORDS: u32 = 14;
        const SLOT_STRIDE_SHIFT: u32 = 6;
        const FLAG_BYTE: u32 = 0x8c;
        const REFILL_FLAG: u8 = 0x10;
        const RATE_WORD: u32 = 0xc;
        const CACHED_VOICE: u32 = 0xc8;
        const DERIVED_EVEN: u32 = 0x124;
        const DERIVED_REST: u32 = 0x128;
        const SRC_BIAS: u32 = 0xc;
        const CAL_RESOLVE: u32 = 1;
        const CAL_CONSUME: u32 = 2;
        const MILLI: f32 = f32::from_bits(0x3a83_126f); // 0.001
        const TWO63_F: f32 = 9223372036854775808.0; // 2^63

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Low dword of the original's `fistp qword` of an integral float:
        /// the value truncated to 32 bits, or zero when the store would
        /// have held the indefinite value (not finite or out of range).
        #[inline(always)]
        fn fistp_low(x: f32) -> u32 {
            if x.is_finite() && x < TWO63_F && x >= -TWO63_F {
                (x as i64) as u32
            } else {
                0
            }
        }

        let cursor = rd32(this.wrapping_add(SLOT_CURSOR));
        let src = rd32(this.wrapping_add(SAMPLE_SRC));
        let slot = this.wrapping_add(cursor.wrapping_shl(SLOT_STRIDE_SHIFT));
        let mut i = 0u32;
        while i < WINDOW_WORDS {
            wr32(
                slot.wrapping_add(SLOT_WINDOW).wrapping_add(i.wrapping_mul(4)),
                rd32(src.wrapping_add(i.wrapping_mul(4))),
            );
            i = i.wrapping_add(1);
        }
        let mut left = count;
        let flags = (this.wrapping_add(FLAG_BYTE) as *const u8).read();
        if count != 0 && flags & REFILL_FLAG != 0 {
            let rate = rd32(this.wrapping_add(RATE_WORD));
            let voice = lf_checker_rt::callee_cdecl!(CAL_RESOLVE, u32, left, rate);
            wr32(this.wrapping_add(CACHED_VOICE), voice);
            let used = lf_checker_rt::callee_stdcall!(CAL_CONSUME, u32, left);
            if (used as i32) > 0 {
                left = left.wrapping_sub(used);
            }
        }
        let rate = rd32(this.wrapping_add(RATE_WORD));
        let scaled = mul(mul(rate as f32, left as f32), MILLI);
        let even = fistp_low(scaled.floor()).wrapping_mul(2);
        wr32(slot.wrapping_add(DERIVED_EVEN), even);
        let rest = rd32(src.wrapping_add(SRC_BIAS)).wrapping_sub(even);
        wr32(slot.wrapping_add(DERIVED_REST), rest);
        let divisor = rd32(this.wrapping_add(SLOT_DIVISOR));
        let next = cursor.wrapping_add(1);
        wr32(this.wrapping_add(SLOT_CURSOR), next % divisor);
        next / divisor
    }
});
