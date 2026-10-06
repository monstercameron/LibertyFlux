// original: 0x00d6c980 filemem_dual_rescale (proposed)

/// Rescale every table span twice, accumulating into two out-words.
///
/// `this` is the file-memory context holding the entry table pointer at
/// `+0x9c` (an array pointer at `+0`, a 16-bit count at `+4`; each entry
/// carries a kind byte at `+1` and a start position at `+0x14`). `bound` is
/// compared against starts as UNSIGNED; `out1` and `out2` are out-words,
/// zeroed on entry and accumulated into. Returns the table end address
/// (`array + count * 4`), including for an empty table.
///
/// Behaviour: walk the entries in order, skipping kind 100. For a middle
/// entry, `len = next_start - start` (wrapping); map the kind through the
/// two small helpers (callees 1 and 2), then `q = (float)(int32)len /
/// (float)(uint32)answer * 33.0`, rounded half away from zero and minus the
/// length, added to `out2`. Then, unless `start >= bound` (unsigned): if
/// `next_start >= bound` (unsigned) the same computation with
/// `len = bound - start` goes to `out1`, else the same length is mapped
/// through a second helper pair and its result goes to `out1`. For the last
/// entry the length is `span - start` where `span` is the global span
/// (`[G+4] - [G]`, wrapping, compared SIGNED: `start >= span` skips the
/// entry), converted as UNSIGNED through a double (unlike the middle
/// lengths, which convert signed), mapped once with the result added to
/// `out2`; then, unless `start >= bound` (unsigned), `len = bound - start`
/// (unsigned conversion) is mapped once more with the result added to
/// `out1`. The divisor always converts unsigned through a double (signed
/// conversion plus 0.0 or 2^32 by the sign bit, narrowed once). Rounding
/// everywhere is half away from zero: compare against 0.0 with the
/// unordered case taking the add side, add or subtract 0.5, truncate toward
/// zero with out-of-range and NaN giving 0x80000000.
///
/// Original: 0x00d6c980 (thiscall, ECX = this, three stack words; both
/// callees are cdecl of one word).
lf_checker_rt::export!(thiscall, rw_00d6c980(this: u32, bound: u32, out1: u32, out2: u32) -> u32 {
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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        /// Map the kind, divide the already-converted length by the
        /// unsigned answer, scale, round half away from zero, subtract the
        /// raw length. Returns the wrapping delta.
        #[inline(always)]
        unsafe fn scaled_delta(kind: u32, len_f: f32, len: u32, scale: f32, half: f32) -> u32 {
            unsafe {
                let key: u32 = lf_checker_rt::callee_cdecl!(CALLEE_KINDMAP, u32, kind);
                let ans: u32 = lf_checker_rt::callee_cdecl!(CALLEE_VALUEMAP, u32, key);
                let mut q = div(len_f, u32_to_f32(ans));
                q = mul(q, scale);
                let adj = if q < 0.0 { sub(q, half) } else { add(q, half) };
                (cvttss2si_exact(adj) as u32).wrapping_sub(len)
            }
        }

        let scale = rdf(lf_checker_rt::relocated(SCALE_ADDR));
        let half = rdf(lf_checker_rt::relocated(HALF_ADDR));
        wr32(out2, 0);
        wr32(out1, 0);
        let table = rd32(this.wrapping_add(TABLE_OFF));
        let count = rd16(table.wrapping_add(COUNT_OFF));
        let array = rd32(table.wrapping_add(ARR_OFF));
        let end = array.wrapping_add(count.wrapping_mul(4));
        if count == 0 {
            return end;
        }
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
                    if (start as i32) < (span as i32) {
                        let len = span.wrapping_sub(start);
                        let d = scaled_delta(kind as u32, u32_to_f32(len), len, scale, half);
                        wr32(out2, rd32(out2).wrapping_add(d));
                        if start < bound {
                            let len2 = bound.wrapping_sub(start);
                            let d2 =
                                scaled_delta(kind as u32, u32_to_f32(len2), len2, scale, half);
                            wr32(out1, rd32(out1).wrapping_add(d2));
                        }
                    }
                } else {
                    let next_start = rd32(rd32(next).wrapping_add(ENTRY_START));
                    let len = next_start.wrapping_sub(start);
                    let len_f = (len as i32) as f32;
                    let d = scaled_delta(kind as u32, len_f, len, scale, half);
                    wr32(out2, rd32(out2).wrapping_add(d));
                    if start < bound {
                        if next_start >= bound {
                            let len2 = bound.wrapping_sub(start);
                            let d2 = scaled_delta(
                                kind as u32,
                                (len2 as i32) as f32,
                                len2,
                                scale,
                                half,
                            );
                            wr32(out1, rd32(out1).wrapping_add(d2));
                        } else {
                            let d2 = scaled_delta(kind as u32, len_f, len, scale, half);
                            wr32(out1, rd32(out1).wrapping_add(d2));
                        }
                    }
                }
            }
            slot = slot.wrapping_add(4);
        }
        end
    }
});
