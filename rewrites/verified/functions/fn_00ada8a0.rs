// original: 0x00ada8a0 seg_interp_dispatch
use lf_checker_rt::{callee_cdecl, export, relocated};

/// Dispatch one interpolation step along a segment, through two scripted
/// sub-handlers.
///
/// Ten cdecl words. a0 is the sample position; a2 and a8 are the two ends of
/// the segment. offset = a0 - a2 and span = a8 - a2 (wrapping integers), and
/// the single-precision fraction t = offset / span with its complement
/// u = 1 - t. a3, a6 and a9 are single-precision words that weight the two
/// channels. a1 and a7 are channel words, a4 an origin word and a5 a value
/// passed through unchanged.
///
/// The anchor is a4 + ((a1 - a4) * offset) / span when a1 equals a7, and
/// a1 + ((a4 - a1) * offset) / span otherwise (integer division truncates
/// toward zero, and the multiplication wraps). Span must be nonzero: the
/// original divides by it on every path and faults when it is zero.
///
/// Whether a2 lies below a8 as signed integers, and whether a1 equals a7,
/// select one of four paths. Every path makes exactly three calls, in the
/// order sub-handler B, sub-handler A, sub-handler B, and returns the answer
/// of the last one; earlier answers are discarded. Each sum keeps the
/// operand order of the original, so a not-a-number operand comes out with
/// the payload the hardware selects.
export!(cdecl, rw_00ada8a0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32) -> u32 {
    // Destination-first single-precision arithmetic. A NaN destination wins,
    // quieted; else a NaN source wins, quieted; else the plain result. The
    // compiler may commute a plain operator and swap which NaN survives.
    fn sse_add(dest: f32, src: f32) -> f32 {
        if dest.is_nan() {
            return f32::from_bits(dest.to_bits() | 0x0040_0000);
        }
        if src.is_nan() {
            return f32::from_bits(src.to_bits() | 0x0040_0000);
        }
        dest + src
    }
    fn sse_mul(dest: f32, src: f32) -> f32 {
        if dest.is_nan() {
            return f32::from_bits(dest.to_bits() | 0x0040_0000);
        }
        if src.is_nan() {
            return f32::from_bits(src.to_bits() | 0x0040_0000);
        }
        dest * src
    }
    unsafe {
        // Callee ids: SUB_A is the wider sub-handler (17 words), SUB_B the
        // narrower one (9 words), both called through the checker's stubs.
        const SUB_A: u32 = 1;
        const SUB_B: u32 = 2;
        // The constant 1.0 the original loads from the image.
        const ONE: u32 = 0x00FE_88E8;
        let one = (relocated(ONE) as *const f32).read_unaligned();

        let offset = (a0 as i32).wrapping_sub(a2 as i32);
        let span = (a8 as i32).wrapping_sub(a2 as i32);
        let t = (offset as f32) / (span as f32);
        let u = one - t;
        let fa3 = f32::from_bits(a3);
        let fa6 = f32::from_bits(a6);
        let fa9 = f32::from_bits(a9);

        // Products and sums shared by the paths, in the original's operand order.
        let u_fa6 = sse_mul(u, fa6);
        let u_fa3 = sse_mul(u, fa3);
        let t_fa9 = sse_mul(fa9, t);
        let t_fa6 = sse_mul(fa6, t);
        let t_fa3 = sse_mul(fa3, t);
        let sum_9_6 = sse_add(t_fa9, u_fa6).to_bits();
        let sum_9_3 = sse_add(t_fa9, u_fa3).to_bits();
        let sum_6_3 = sse_add(t_fa6, u_fa3).to_bits();
        let ascending = (a2 as i32) < (a8 as i32);

        if a1 == a7 {
            let anchor = (a4 as i32)
                .wrapping_add(((a1 as i32).wrapping_sub(a4 as i32)).wrapping_mul(offset) / span)
                as u32;
            if ascending {
                let l1 = sse_add(u_fa6, t_fa9).to_bits();
                let l2 = sse_add(u_fa6, t_fa3).to_bits();
                let _: u32 = callee_cdecl!(SUB_B, u32, anchor, a2, l2, a4, a5, a6, anchor, a0, l1);
                let m1 = l1;
                let m2 = sse_add(u_fa3, t_fa9).to_bits();
                let m3 = l2;
                let _: u32 = callee_cdecl!(
                    SUB_A, u32, a1, anchor, a2, a0, a3, m3, m2, m1, anchor, a2, l2, a4, a5, a6,
                    anchor, a0, l1
                );
            } else {
                let p = sse_add(t_fa9, u_fa6).to_bits();
                let q = sse_add(t_fa3, u_fa6).to_bits();
                let o2 = sse_add(t_fa9, u_fa3).to_bits();
                let _: u32 = callee_cdecl!(SUB_B, u32, anchor, a2, q, a4, a5, a6, anchor, a0, p);
                let _: u32 = callee_cdecl!(
                    SUB_A, u32, a1, anchor, a0, a2, o2, p, a3, q, anchor, a2, q, a4, a5, a6,
                    anchor, a0, p
                );
            }
            callee_cdecl!(SUB_B, u32, a1, a0, sum_9_3, anchor, a0, sum_9_6, a7, a8, a9)
        } else {
            let anchor = (a1 as i32)
                .wrapping_add(((a4 as i32).wrapping_sub(a1 as i32)).wrapping_mul(offset) / span)
                as u32;
            let _: u32 = callee_cdecl!(
                SUB_B, u32, a1, a2, a3, anchor, a2, sum_6_3, anchor, a0, sum_9_3
            );
            if ascending {
                let _: u32 = callee_cdecl!(
                    SUB_A, u32, anchor, a4, a2, a0, sum_6_3, a6, sum_9_3, sum_9_6, a1, a2, a3,
                    anchor, a2, sum_6_3, anchor, a0, sum_9_3
                );
            } else {
                let _: u32 = callee_cdecl!(
                    SUB_A, u32, anchor, a4, a0, a2, sum_9_3, sum_9_3, sum_6_3, a6, a1, a2, a3,
                    anchor, a2, sum_6_3, anchor, a0, sum_9_3
                );
            }
            callee_cdecl!(SUB_B, u32, anchor, a0, sum_9_3, a4, a0, sum_9_6, a7, a8, a9)
        }
    }
});
