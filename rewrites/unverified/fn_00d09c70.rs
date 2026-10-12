// original: 0x00D09C70 CTaskComplexCombatAdditionalTask::vf19

/// Seed a combat-extra task's timer from a random draw, then chain two steps.
///
/// Draws an integer (patched thiscall callee), converts it to float exactly,
/// scales it by two read-only constants and stores the result at `+0x38`,
/// then runs the plan step (patched thiscall callee) on the incoming word and
/// feeds its answer with the incoming word into the apply step (patched
/// thiscall callee), whose answer is returned.
///
/// Original: 0x00D09C70 (thiscall, one stack word, callee pops).
lf_checker_rt::export!(thiscall, rw_00d09c70(this: u32, a1: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        const DRAW: u32 = 1;
        const PLAN: u32 = 2;
        const APPLY: u32 = 3;
        const K1: u32 = 0x00FE8684;
        const K2: u32 = 0x00FE8A24;
        const TIMER: u32 = 0x38;
        let q = lf_checker_rt::callee_thiscall!(DRAW, u32, this);
        let k1 = f32::from_bits((lf_checker_rt::relocated(K1) as *const u32).read_unaligned());
        let k2 = f32::from_bits((lf_checker_rt::relocated(K2) as *const u32).read_unaligned());
        let x = mul(mul((q as i32) as f32, k1), k2);
        ((this + TIMER) as *mut u32).write_unaligned(add(x, k2).to_bits());
        let plan = lf_checker_rt::callee_thiscall!(PLAN, u32, this, a1, 1);
        lf_checker_rt::callee_thiscall!(APPLY, u32, this, a1, plan)
    }
});
