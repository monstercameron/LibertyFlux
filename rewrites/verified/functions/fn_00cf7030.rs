// original: 0x00cf7030 climb_task_vector_commit (proposed)

/// Commits a climb task's solved vector: classifies the scale argument twice
/// through float-in-xmm0 helpers (the helpers' answers are ignored; the stub
/// preserves xmm0 like the originals, and the comparison checks the passed
/// values), then combines the inputs in order: `t = scale*mult + v0`,
/// `x = t - v0` stored at `+0x40`, `y = (v2 + extra) - v2` at `+0x48`,
/// `w` copied from an uninitialized frame word (modelled as zero) at `+0x4c`,
/// `z = (v1 - scale*mult) - v1` at `+0x44`. Sets bit 1 of `+0x89` and runs
/// the finish callee, returning its result.
///
/// Original: 0x00cf7030 (thiscall: ecx holds the object, four stack words:
// roughly (vec, scale, mult, extra)).
lf_checker_rt::export!(thiscall, rw_00cf7030(this: u32, vec: u32, scale: u32, mult: u32, extra: u32) -> u32 {
    unsafe {
        const CLASS_A: u32 = 1;
        const CLASS_B: u32 = 2;
        const FINISH_CALLEE: u32 = 3;
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
        let s = f32::from_bits(scale);
        let m = f32::from_bits(mult);
        let v0 = f32::from_bits((vec as *const u32).read_unaligned());
        lf_checker_rt::callee_cdecl!(CLASS_A, u32, scale);
        let t = add(mul(s, m), v0);
        lf_checker_rt::callee_cdecl!(CLASS_B, u32, scale);
        let v1 = f32::from_bits(((vec + 4) as *const u32).read_unaligned());
        let v2 = f32::from_bits(((vec + 8) as *const u32).read_unaligned());
        let e = f32::from_bits(extra);
        let t1 = mul(s, m);
        let y = sub(add(v2, e), v2);
        let x = sub(t, v0);
        let z = sub(sub(v1, t1), v1);
        // Uninitialized frame word above the aligned area; the contract fills
        // uninitialized stack with zero on both sides.
        let w = f32::from_bits(0);
        ((this + 0x40) as *mut u32).write_unaligned(x.to_bits());
        ((this + 0x48) as *mut u32).write_unaligned(y.to_bits());
        ((this + 0x4c) as *mut u32).write_unaligned(w.to_bits());
        ((this + 0x44) as *mut u32).write_unaligned(z.to_bits());
        let flags = ((this + 0x89) as *const u8).read();
        ((this + 0x89) as *mut u8).write(flags | 2);
        lf_checker_rt::callee_thiscall!(FINISH_CALLEE, u32, this)
    }
});
