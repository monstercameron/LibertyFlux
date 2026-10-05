// original: 0x00b739a0 task_ctor_random_duration (proposed)

/// Build a random-duration task and optionally tune its blend weight.
///
/// `this` is the task object (int fields at `+A`/`+B` forwarded to the
/// creator, qualifier object at `+QUAL`, qualifier key at `+KEY`, created
/// task stored at `+TASK`); `arg` is the host-side record (callback
/// object at `+CALLBACK`).
///
/// Behaviour: callee 0 initialises from the host record, callee 1 creates
/// the task from (`+B`, `+A`, 8.0, -1), callee 2 attaches it. Callee 3
/// answers a random integer 0..=0x7FFF; it is converted to float and
/// scaled (`rng*K_RNG_A*K_RNG_B + K_RNG_C`, in that operand order) into
/// `+DURATION`. Callee 4 then qualifies (`+KEY` against `+QUAL`): a null
/// answer ends the function, otherwise its `+THRESH` word must exceed
/// `K_GATE` (strictly, NaN ends it too). Callee 5 re-checks through the
/// task's link (`+LINK` when `+HAS_LINK` is 1, else null): zero ends it.
/// Two sampling calls through callee 6 (flags `0x10000`, `0x20000`) each
/// answer success in the low byte; two failures end it. Otherwise a final
/// weight is computed from the threshold `x`, the second sample slot and
/// the first sample slot `b` (zero here: the original's slot is either
/// explicitly zeroed or uninitialised stack, and the contract pins the
/// stack fill to zero): `xmm1 = (s - b) * (K_ONE - min(K_ONE, x)) + b`
/// with `s = K_ONE` when the second sample failed and the incoming `arg`
/// bits otherwise, then passed to callee 7. The two sample slots live in
/// the caller's argument area, so the stack check is off for this
/// function; every value flowing out of them is still compared as a call
/// argument.
///
/// Returns the last callee answer on each path (zero, the qualifier, the
/// second sample answer, or callee 7's answer).
///
/// Original: 0x00b739a0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b739a0(this: u32, arg: u32) -> u32 {
    unsafe {
        const A: u32 = 0x1c;
        const B: u32 = 0x20;
        const TASK: u32 = 0x18;
        const KEY: u32 = 0x28;
        const QUAL: u32 = 0x24;
        const CALLBACK: u32 = 0x78;
        const LINK: u32 = 0x40;
        const HAS_LINK: u32 = 0x44;
        const THRESH: u32 = 0x1c;
        const DURATION: u32 = 0x54;
        const K_RNG_A: u32 = 0x00fe8684;
        const K_RNG_B: u32 = 0x00fe87d8;
        const K_RNG_C: u32 = 0x00fe88bc;
        const K_GATE: u32 = 0x00fe88c4;
        const K_ONE: u32 = 0x00fe88e8;
        const VTBL: u32 = 0x00b6fde0;
        const EIGHT: u32 = 0x4100_0000;
        const ONE: u32 = 0x3f80_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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

        lf_checker_rt::callee_thiscall!(0, u32, this, arg);
        let task: u32 = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            rd32(arg + CALLBACK),
            rd32(this + B),
            rd32(this + A),
            EIGHT,
            0xffff_ffff
        );
        wr32(this + TASK, task);
        lf_checker_rt::callee_thiscall!(
            2,
            u32,
            task,
            1,
            lf_checker_rt::relocated(VTBL),
            this
        );
        let rng: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
        let f = add(
            mul(
                mul(
                    (rng as i32) as f32,
                    rdf(lf_checker_rt::relocated(K_RNG_A)),
                ),
                rdf(lf_checker_rt::relocated(K_RNG_B)),
            ),
            rdf(lf_checker_rt::relocated(K_RNG_C)),
        );
        wrf(task + DURATION, f);
        let qual: u32 =
            lf_checker_rt::callee_thiscall!(4, u32, rd32(this + QUAL), rd32(this + KEY));
        if qual == 0 {
            return 0;
        }
        let x = f32::from_bits(rd32(qual + THRESH));
        if !(rdf(lf_checker_rt::relocated(K_GATE)) > x) {
            return qual;
        }
        let link = if rd16(task + HAS_LINK) == 1 {
            rd32(task + LINK)
        } else {
            0
        };
        let recheck: u32 = lf_checker_rt::callee_thiscall!(5, u32, link, 0x80, 0);
        if recheck == 0 {
            return 0;
        }
        // Sample slots: addresses are skipped in the comparison; only the
        // low-byte answers are observed.
        let mut dummy = 0u32;
        let task = rd32(this + TASK);
        let s1: u32 = lf_checker_rt::callee_thiscall!(
            6,
            u32,
            task,
            0x10000,
            &mut dummy as *mut u32 as u32,
            0,
            ONE
        );
        let mut fails = 0u32;
        // First slot: zeroed on failure, else uninitialised stack (the
        // contract's stack fill, zero).
        let b = 0.0f32;
        if s1 & 0xff == 0 {
            fails += 1;
        }
        let s2: u32 = lf_checker_rt::callee_thiscall!(
            6,
            u32,
            task,
            0x20000,
            &mut dummy as *mut u32 as u32,
            0,
            ONE
        );
        let k_one = rdf(lf_checker_rt::relocated(K_ONE));
        let mut s = f32::from_bits(arg);
        if s2 & 0xff == 0 {
            s = k_one;
            fails += 1;
        }
        if fails >= 2 {
            return s2;
        }
        let x3 = if k_one > x { x } else { k_one };
        let t = mul(sub(s, b), sub(k_one, x3));
        let w = add(t, b);
        lf_checker_rt::callee_thiscall!(7, u32, task, w.to_bits())
    }
});
