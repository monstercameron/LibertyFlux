// original: 0x0088D960 rage::audVoiceSoft::vf8
/// Refresh one soft-voice ring lane and return the next lane index.
///
/// Copies 14 words from the voice's source object into the current lane,
/// optionally refines the sample count through two engine calls, derives a
/// scaled sample value from the voice rate and count with round-to-nearest
/// float magic, stores twice its truncated integer part, records the
/// remainder against the source, and steps the ring index. The return value
/// is the division quotient of the index step.
export!(thiscall, rw_88d960(this: *mut u8, count: u32) -> u32 {
    unsafe {
        const RATE: usize = 0xC;
        const AUX: usize = 0x10;
        const SRC: usize = 0x14;
        const FLAGS: usize = 0x8C;
        const INDEX: usize = 0x118;
        const COUNT: usize = 0x120;
        const OUT: usize = 0x128;
        const LANE_STRIDE: usize = 64;
        const LANE_BUF: usize = 0x90;
        const LANE_POS: usize = 0xC8;
        const LANE_REM: usize = 0xCC;

        #[inline]
        unsafe fn rd(base: *mut u8, off: usize) -> u32 {
            *(base.add(off) as *mut u32)
        }
        #[inline]
        unsafe fn wr(base: *mut u8, off: usize, v: u32) {
            *(base.add(off) as *mut u32) = v;
        }
        // Float constants from the game's read-only data (verified file bytes;
        // the original loads these exact words).
        const K_MILLI: f32 = f32::from_bits(0x3A83126F); // 0.001
        const K_2P23: f32 = f32::from_bits(0x4B000000); // 2^23
        const K_ONE: f32 = 1.0;
        const K_SIGN_MASK: u32 = 0x80000000; // folded -0.0 load
        const T_ADJ: [f64; 2] = [0.0, 4294967296.0];
        // Unsigned int to float through a double, mirroring the original's
        // signed-convert plus table adjustment step for step.
        #[inline]
        fn u32_to_f32(v: u32) -> f32 {
            ((v as i32) as f64 + T_ADJ[(v >> 31) as usize]) as f32
        }
        // Truncate a float to 32 bits the way the original's float-to-int64
        // store with truncation does: out-of-range and NaN yield the
        // indefinite value, whose low word is zero. No f32 falls strictly
        // between i64::MAX and 2^63 (or below -2^63), so the `as` cast never
        // saturates here.
        #[inline]
        fn trunc_f32_to_i64_lo(v: f32) -> u32 {
            const TWO63: f32 = 9223372036854775808.0; // 2^63, exact in f32
            if v.is_nan() || v >= TWO63 || v < -TWO63 {
                0
            } else {
                (v as i64) as u32
            }
        }

        let idx = rd(this, INDEX);
        let lane = (idx as usize).wrapping_mul(LANE_STRIDE);
        // Lane refresh: 14 words from the source object.
        let src = rd(this, SRC) as *mut u32;
        let dst = this.add(LANE_BUF + lane) as *mut u32;
        core::ptr::copy_nonoverlapping(src, dst, 14);

        let mut n = count;
        if n != 0 && (*this.add(FLAGS) & 0x10) != 0 {
            let rate = rd(this, RATE);
            // Stack layout is [n, rate] (rate pushed first).
            let refined: u32 = callee_cdecl!(1, u32, n, rate);
            wr(this, OUT, refined);
            let took: u32 = callee_thiscall!(2, u32, rd(this, AUX), n);
            if (took as i32) > 0 {
                n = n.wrapping_sub(took);
            }
        }

        // Scaled sample value with round-to-nearest magic.
        let mut x5 = u32_to_f32(rd(this, RATE)) * u32_to_f32(n);
        x5 *= K_MILLI;
        let sign = f32::from_bits(x5.to_bits() & K_SIGN_MASK);
        let x2 = if f32::from_bits(x5.to_bits() & 0x7FFFFFFF) < K_2P23 {
            f32::from_bits(K_2P23.to_bits() | sign.to_bits())
        } else {
            sign
        };
        let mut x1 = x5 + x2;
        x1 -= x2;
        let mut x0 = x1 - x5;
        // Not-less-or-equal (the `cmpnless` mnemonic is NLE, imm 6: true for
        // greater-than or unordered), exactly like `!(<=)`.
        x0 = if !(x0 <= sign) { K_ONE } else { 0.0 };
        x1 -= x0;

        let pos = trunc_f32_to_i64_lo(x1).wrapping_add(trunc_f32_to_i64_lo(x1));
        let idx2 = rd(this, INDEX);
        let lane2 = (idx2 as usize).wrapping_mul(LANE_STRIDE);
        wr(this, LANE_POS + lane2, pos);
        let src_field = *((rd(this, SRC) as *mut u8).add(0xC) as *mut u32);
        wr(this, LANE_REM + lane2, src_field.wrapping_sub(pos));

        let next = rd(this, INDEX).wrapping_add(1);
        let divisor = rd(this, COUNT);
        wr(this, INDEX, next % divisor);
        next / divisor
    }
});
