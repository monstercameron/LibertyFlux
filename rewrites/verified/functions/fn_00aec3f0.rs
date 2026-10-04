// original: 0x00aec3f0 timing_score_and_store (proposed)

/// Score one timing row and store the scaled result.
///
/// `ctx` points to a context whose words at `+0xb4` (object) and `+0xb8`
/// (argument) feed the row-index callee. The row index times 96 plus the
/// vector base at object `+0x80` selects nine floats (three triples half a
/// row apart); the score callee rates the row, and unless the score strictly
/// exceeds the stored threshold the function returns the score bits. On
/// success it fetches a four-word vector through the fill callee into its
/// own frame, normalises the cross product of the row triples (a zero
/// squared length keeps a zero factor instead of dividing), scales it by the
/// gain global, adds the fetched vector, stores the four words to the output
/// globals and records the commit callee's answer.
///
/// Arguments: one stack word (`ctx`). Cdecl. Early return (gate byte clear)
/// returns the entry accumulator untouched, so the contract fixes it to 0.
/// The middle return carries the score callee's float bits (the stub leaves
/// them in the accumulator too). Float operation order is the original's.
lf_checker_rt::export!(cdecl, rw_00aec3f0(ctx: u32) -> u32 {
    unsafe {
        const OBJ: u32 = 0xb4;
        const ARGW: u32 = 0xb8;
        const VEC_BASE: u32 = 0x80;
        const ROW_STRIDE: u32 = 96;
        const GATE: u32 = 0x0103f9a8;
        const THRESH: u32 = 0x0159b764;
        const GAIN: u32 = 0x0103f73c;
        const CTX_WORD: u32 = 0x012fb214;
        const ONE: u32 = 0x00fe88e8;
        const OUT0: u32 = 0x015c3bc0;
        const OUT1: u32 = 0x015c3bc4;
        const OUT2: u32 = 0x015c3bc8;
        const OUT3: u32 = 0x015c3bcc;
        const RESULT: u32 = 0x0103f6d0;
        const ROW_PTR: u32 = 0x015b2d00;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        if (lf_checker_rt::global::<u8>(GATE)).read() == 0 {
            return 0;
        }
        let obj = rd32(ctx + OBJ);
        let idx: u32 = lf_checker_rt::callee_thiscall!(1, u32, obj, 0u32, rd32(ctx + ARGW));
        let row = idx.wrapping_mul(3).wrapping_mul(32).wrapping_add(rd32(obj + VEC_BASE));
        let score: f32 =
            lf_checker_rt::callee_thiscall!(2, f32, row, 1u32, lf_checker_rt::relocated(ROW_PTR));
        let thresh = (lf_checker_rt::global::<f32>(THRESH)).read_unaligned();
        if !(score > thresh) {
            return score.to_bits();
        }
        (lf_checker_rt::global::<f32>(THRESH)).write_unaligned(score);
        let mut v = [0u32; 5];
        lf_checker_rt::callee_thiscall!(3, u32, row, v.as_mut_ptr() as u32);
        // v3 is the fourth fill word: the final read happens after one more
        // push, so [esp+0x20] there is [buf+12] here. The fifth word is never read.
        let (v0, v1, v2, v3) = (
            f32::from_bits(v[0]),
            f32::from_bits(v[1]),
            f32::from_bits(v[2]),
            f32::from_bits(v[3]),
        );
        let (s0, s4, s8) = (rdf(row), rdf(row + 4), rdf(row + 8));
        let (s10, s14, s18) = (rdf(row + 0x10), rdf(row + 0x14), rdf(row + 0x18));
        let (s20, s24, s28) = (rdf(row + 0x20), rdf(row + 0x24), rdf(row + 0x28));
        let a = sub(s8, s18);
        let b = sub(s18, s28);
        let c = sub(s4, s14);
        let d = sub(s14, s24);
        let e = sub(s0, s10);
        let f = sub(s10, s20);
        let n1 = sub(mul(b, c), mul(d, a));
        let b2 = mul(b, e);
        let n0 = sub(mul(f, a), b2);
        let n2 = sub(mul(d, e), mul(f, c));
        let len2 = add(add(mul(n0, n0), mul(n1, n1)), mul(n2, n2));
        // The original skips the divide exactly when the squared length
        // compares equal to zero (either sign), leaving a zero factor.
        let inv = if len2 == 0.0 {
            0.0
        } else {
            let one = (lf_checker_rt::global::<f32>(ONE)).read_unaligned();
            core::hint::black_box(one) / core::hint::black_box(len2.sqrt())
        };
        let gain = (lf_checker_rt::global::<f32>(GAIN)).read_unaligned();
        let o0 = add(v2, mul(mul(n2, inv), gain));
        let o1 = add(v0, mul(mul(n1, inv), gain));
        let o2 = add(v1, mul(mul(n0, inv), gain));
        (lf_checker_rt::global::<f32>(OUT2)).write_unaligned(o0);
        (lf_checker_rt::global::<f32>(OUT0)).write_unaligned(o1);
        (lf_checker_rt::global::<f32>(OUT1)).write_unaligned(o2);
        (lf_checker_rt::global::<f32>(OUT3)).write_unaligned(v3);
        let ans: u32 = lf_checker_rt::callee_thiscall!(
            4,
            u32,
            (lf_checker_rt::global::<u32>(CTX_WORD)).read_unaligned(),
            obj
        );
        (lf_checker_rt::global::<u32>(RESULT)).write_unaligned(ans);
        ans
    }
});
