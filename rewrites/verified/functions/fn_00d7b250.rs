// original: 0x00D7B250 convert_byte_dual_filter_scale (proposed)

/// Scale a byte argument through two float filters into an output pair.
///
/// Converts the low byte of `a1` to float, scales by 0.02453125 and feeds
/// the result to two helpers taking their argument in `xmm0`; each answer
/// is scaled by 0.7 and the pair is stored to `*a3` and `*(a3 + 4)`. The
/// pair is then scaled by 0.15 when the low byte of `a2` is non-zero,
/// else when bit 1 of the word at `[table[a0] + 0x94]` is set (the table
/// lives at `0x01295cd8`). Returns the second value's bits on the
/// direct-scale path, and the probed table word shifted right by 1 on the
/// table path (the probe leaves its result in `eax`). Cdecl, four stack
/// words. Float operation order is the original's.
use lf_checker_rt::{callee_cdecl, export, relocated};

const FILTER_A: u32 = 1;
const FILTER_B: u32 = 2;

export!(cdecl, rw_00d7b250(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        const BYTE_SCALE: f32 = f32::from_bits(0x3cc8_f5c3); // 0.02453125
        const ANSWER_SCALE: f32 = f32::from_bits(0x3f33_3333); // 0.7
        const OUT_SCALE: f32 = f32::from_bits(0x3e19_999a); // 0.15
        const TABLE: u32 = 0x01295cd8;
        const PROBE_OFF: u32 = 0x94;
        let f = mul((a1 & 0xff) as f32, BYTE_SCALE);
        let r1 = mul(f32::from_bits(callee_cdecl!(FILTER_A, u32, f.to_bits())), ANSWER_SCALE);
        let r2 = mul(f32::from_bits(callee_cdecl!(FILTER_B, u32, f.to_bits())), ANSWER_SCALE);
        (a3 as *mut u32).write_unaligned(r1.to_bits());
        ((a3 + 4) as *mut u32).write_unaligned(r2.to_bits());
        if (a2 & 0xff) != 0 {
            let o0 = f32::from_bits((a3 as *const u32).read_unaligned());
            (a3 as *mut u32).write_unaligned(mul(o0, OUT_SCALE).to_bits());
            let o1 = f32::from_bits(((a3 + 4) as *const u32).read_unaligned());
            ((a3 + 4) as *mut u32).write_unaligned(mul(o1, OUT_SCALE).to_bits());
            return r2.to_bits();
        }
        let entry = ((relocated(TABLE) + a0.wrapping_mul(4)) as *const u32).read_unaligned();
        let v = ((entry + PROBE_OFF) as *const u32).read_unaligned();
        let probed = v >> 1;
        if probed & 1 != 0 {
            let o0 = f32::from_bits((a3 as *const u32).read_unaligned());
            (a3 as *mut u32).write_unaligned(mul(o0, OUT_SCALE).to_bits());
            let o1 = f32::from_bits(((a3 + 4) as *const u32).read_unaligned());
            ((a3 + 4) as *mut u32).write_unaligned(mul(o1, OUT_SCALE).to_bits());
        }
        probed
    }
});
