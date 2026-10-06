// original: 0x00d6cee0 filemem_covered_rescale (proposed)

/// Rescale the table-covered length below a limit and add it to the limit.
///
/// `this` is the file-memory context holding the entry table pointer at
/// `+0x9c` (an array pointer at `+0`, a 16-bit count at `+4`; each entry
/// carries a kind byte at `+1` and a start position at `+0x14`). `limit` is
/// the position up to which covered length is measured; the second stack
/// argument is never read. All position comparisons are SIGNED.
///
/// Behaviour: with no table or an empty one, return `limit`. Otherwise walk
/// the entries in order, skipping kind 100. For a middle entry with
/// `start > limit` (signed) skip it; else its length is
/// `min(next_start, limit) - start` (signed choice, wrapping difference).
/// For the last entry the length is `limit - start`, taken only when
/// `start` is below both the global span (`[G+4] - [G]`, wrapping, compared
/// signed) and the running `limit` value; a skipped last entry leaves the
/// running value alone (the original jumps past its only reload). Each taken
/// length is mapped through the two small helpers (callees 1 and 2) fed
/// with the kind, then `q = (float)(int32)length / (float)(uint32)answer *
/// 33.0` (the divisor goes through the double dance: signed conversion to
/// double plus 0.0 or 2^32 by the sign bit, narrowed once), rounded half
/// away from zero (compare against 0.0 with the unordered case taking the
/// add side, add or subtract 0.5, truncate toward zero with out-of-range
/// and NaN giving 0x80000000), minus the length, accumulated wrapping.
/// Return `limit + accumulator`.
///
/// Original: 0x00d6cee0 (thiscall, ECX = this, two stack words of which the
/// second is unread; both callees are cdecl of one word).
lf_checker_rt::export!(thiscall, rw_00d6cee0(this: u32, limit: u32, _unused: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x9C;
        const ARR_OFF: u32 = 0x00;
        const COUNT_OFF: u32 = 0x04;
        const ENTRY_KIND: u32 = 0x01;
        const ENTRY_START: u32 = 0x14;
        const KIND_SKIP: u8 = 100;
        const SCALE_ADDR: u32 = 0x00E758F8;
        const HALF_ADDR: u32 = 0x00FE8830;
        const U32_BIAS_ADDR: u32 = 0x00FE8F50;
        const GSPAN_LO: u32 = 0x011F7028;
        const GSPAN_HI: u32 = 0x011F702C;
        const CALLEE_KINDMAP: u32 = 1;
        const CALLEE_VALUEMAP: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// Unsigned int to float through a double, exactly as the
        /// original's movd/cvtdq2pd/addsd-bias/cvtpd2ps sequence.
        #[inline(always)]
        unsafe fn u32_to_f32(x: u32) -> f32 {
            unsafe {
                let widened = (x as i32) as f64;
                let bias_at = lf_checker_rt::relocated(U32_BIAS_ADDR) + (x >> 31) * 8;
                let bias = (bias_at as *const f64).read_unaligned();
                (core::hint::black_box(widened) + core::hint::black_box(bias)) as f32
            }
        }
        /// Bit-exact `cvttss2si`.
        #[inline(always)]
        fn cvttss2si_exact(x: f32) -> i32 {
            if x.is_nan() {
                return i32::MIN;
            }
            let t = x.trunc();
            if t <= -2147483648.0 || t >= 2147483648.0 {
                i32::MIN
            } else {
                t as i32
            }
        }
        /// One entry's scaled contribution: map the kind, divide the
        /// signed length by the unsigned answer, scale, round half away
        /// from zero, subtract the length. Returns the wrapping delta.
        #[inline(always)]
        unsafe fn scaled_delta(kind: u32, len: u32, scale: f32, half: f32) -> u32 {
            unsafe {
                let key: u32 = lf_checker_rt::callee_cdecl!(CALLEE_KINDMAP, u32, kind);
                let ans: u32 = lf_checker_rt::callee_cdecl!(CALLEE_VALUEMAP, u32, key);
                let mut q = div((len as i32) as f32, u32_to_f32(ans));
                q = mul(q, scale);
                let adj = if q < 0.0 { sub(q, half) } else { add(q, half) };
                (cvttss2si_exact(adj) as u32).wrapping_sub(len)
            }
        }

        let scale = rdf(lf_checker_rt::relocated(SCALE_ADDR));
        let half = rdf(lf_checker_rt::relocated(HALF_ADDR));
        let table = rd32(this.wrapping_add(TABLE_OFF));
        if table == 0 {
            return limit;
        }
        let count = rd16(table.wrapping_add(COUNT_OFF));
        if count == 0 {
            return limit;
        }
        let array = rd32(table.wrapping_add(ARR_OFF));
        let end = array.wrapping_add(count.wrapping_mul(4));
        let mut acc: u32 = 0;
        let mut running = limit;
        let mut slot = array;
        while slot != end {
            let entry = rd32(slot);
            let kind = rd8(entry.wrapping_add(ENTRY_KIND));
            if kind != KIND_SKIP {
                let start = rd32(entry.wrapping_add(ENTRY_START));
                let next = slot.wrapping_add(4);
                if next == end {
                    let span = rd32(lf_checker_rt::relocated(GSPAN_HI))
                        .wrapping_sub(rd32(lf_checker_rt::relocated(GSPAN_LO)));
                    if (start as i32) < (span as i32) && (start as i32) < (running as i32) {
                        let len = running.wrapping_sub(start);
                        acc = acc.wrapping_add(scaled_delta(kind as u32, len, scale, half));
                    }
                    // A skipped last entry leaves the running value alone:
                    // the original jumps past its only reload here.
                } else {
                    let next_start = rd32(rd32(next).wrapping_add(ENTRY_START));
                    if (start as i32) > (limit as i32) {
                        running = limit;
                    } else {
                        let len = if (next_start as i32) < (limit as i32) {
                            next_start.wrapping_sub(start)
                        } else {
                            limit.wrapping_sub(start)
                        };
                        acc = acc.wrapping_add(scaled_delta(kind as u32, len, scale, half));
                        running = limit;
                    }
                }
            }
            slot = slot.wrapping_add(4);
        }
        acc.wrapping_add(running)
    }
});
