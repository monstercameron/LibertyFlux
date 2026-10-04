// original: 0x009F9660 stat_trip_accumulate (proposed)

/// Accumulate one trip sample into the running counters, or flush them.
///
/// `obj` points to a tracker whose mode word at `+0x1304` selects the path.
/// Only mode 0 with a non-positive eligibility answer from the check callee
/// accumulates; every other mode, and a positive answer, flushes.
///
/// Accumulate: add the low word of the chop-converted product of two global
/// rate factors to the sample counter, add the distance between the tracker's
/// anchor point (at the linked point plus 0x30, or at `+0x10` when the link
/// is null) and its reference point (at `+0x200`) to the distance total,
/// then combine the linked point's coordinates through a callee returning a
/// float and keep the larger of the stored peak and the adjusted value.
/// Flush (when the sample counter is non-zero): emit the peak and the total
/// through an emit callee with two fixed stat ids, record float 1.0 against
/// a third id through the stat callee, and clear the three counters.
///
/// The accumulate path leaves scratch in its own incoming argument slot,
/// which a safe rewrite cannot address; the contract switches the stack
/// comparison off for this function (see results `narrowed`).
///
/// Original: cdecl, one stack word (object pointer), no meaningful return.
lf_checker_rt::export!(cdecl, rw_009F9660(obj: u32) -> u32 {
    unsafe {
        const OBJ_MODE: u32 = 0x1304;
        const OBJ_LINK: u32 = 0x20;
        const OBJ_LOCAL_PT: u32 = 0x10;
        const OBJ_REF_PT: u32 = 0x200;
        const LINK_PT_DX: u32 = 0x30;
        const PT_X: u32 = 0x30;
        const PT_Y: u32 = 0x34;
        const PT_Z: u32 = 0x38;
        const RATE_A: u32 = 0x11735bc;
        const RATE_B: u32 = 0xfe8c58;
        const SAMPLE_COUNT: u32 = 0x12b6270;
        const DIST_TOTAL: u32 = 0x12b6274;
        const PEAK_VALUE: u32 = 0x12b6278;
        const EMIT_PEAK_STAT: u32 = 0x58;
        const EMIT_TOTAL_STAT: u32 = 0x57;
        const FLUSH_STAT: u32 = 0x10a;
        const ONE_BITS: u32 = 0x3f80_0000;
        const CHECK_CALLEE: u32 = 0;
        const COMBINE_CALLEE: u32 = 1;
        const EMIT_CALLEE: u32 = 2;
        const STAT_CALLEE: u32 = 3;

        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Low word of the 64-bit chop-converted value, as the original's
        /// `fistp qword` plus low-word load computes it: NaN and
        /// out-of-range values yield the indefinite's low word, 0.
        fn fistp_low(x: f32) -> u32 {
            if x.is_nan() || x >= 9223372036854775808.0 || x < -9223372036854775808.0 {
                0
            } else {
                (x as i64) as u32
            }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let mode = rd32(obj + OBJ_MODE);
        let mut accumulate = false;
        if mode == 0 {
            let eligible = lf_checker_rt::callee_thiscall!(CHECK_CALLEE, u32, obj) as i32;
            if eligible <= 0 {
                accumulate = true;
            }
        }
        if !accumulate {
            if rd32(lf_checker_rt::relocated(SAMPLE_COUNT)) == 0 {
                return 0;
            }
            let peak = rd32(lf_checker_rt::relocated(PEAK_VALUE));
            lf_checker_rt::callee_cdecl!(EMIT_CALLEE, u32, EMIT_PEAK_STAT, peak);
            let total = rd32(lf_checker_rt::relocated(DIST_TOTAL));
            lf_checker_rt::callee_cdecl!(EMIT_CALLEE, u32, EMIT_TOTAL_STAT, total);
            lf_checker_rt::callee_cdecl!(STAT_CALLEE, u32, FLUSH_STAT, ONE_BITS);
            wr32(lf_checker_rt::relocated(SAMPLE_COUNT), 0);
            wr32(lf_checker_rt::relocated(DIST_TOTAL), 0);
            wr32(lf_checker_rt::relocated(PEAK_VALUE), 0);
            return 0;
        }

        let product = mul(
            rdf(lf_checker_rt::relocated(RATE_A)),
            rdf(lf_checker_rt::relocated(RATE_B)),
        );
        let count_addr = lf_checker_rt::relocated(SAMPLE_COUNT);
        wr32(count_addr, rd32(count_addr).wrapping_add(fistp_low(product)));

        let link = rd32(obj + OBJ_LINK);
        let anchor = if link != 0 {
            link.wrapping_add(LINK_PT_DX)
        } else {
            obj.wrapping_add(OBJ_LOCAL_PT)
        };
        let dx = sub(rdf(anchor), rdf(obj + OBJ_REF_PT));
        let dy = sub(rdf(anchor + 4), rdf(obj + OBJ_REF_PT + 4));
        let dz = sub(rdf(anchor + 8), rdf(obj + OBJ_REF_PT + 8));
        let dist2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        let dist = dist2.sqrt();
        let total_addr = lf_checker_rt::relocated(DIST_TOTAL);
        wr32(total_addr, add(dist, rdf(total_addr)).to_bits());

        let link2 = rd32(obj + OBJ_LINK);
        let x = rdf(link2 + PT_Z);
        let y = rdf(link2 + PT_Y);
        let z = rdf(link2 + PT_X);
        let combined: f32 = lf_checker_rt::callee_cdecl!(
            COMBINE_CALLEE,
            f32,
            z.to_bits(),
            y.to_bits(),
            x.to_bits(),
            0,
            0,
            4
        );
        let adjusted = sub(x, combined);
        let peak_addr = lf_checker_rt::relocated(PEAK_VALUE);
        if !(rdf(peak_addr) > adjusted) {
            wr32(peak_addr, adjusted.to_bits());
        }
        0
    }
});
