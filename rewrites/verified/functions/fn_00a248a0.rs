// original: 0x00a248a0 task_heading_update (proposed)

/// Advance the heading block at `this` from two task objects and a rate.
///
/// `obj` and `aux` are task objects (either may be null), `rate` is a float
/// added to the word at `this + 0x158`, and `blend`'s bits travel verbatim
/// into the second callee. The function:
///
/// 1. Runs a lookup callee on `obj`, adds `rate` to `this + 0x158`, then runs
///    a measure callee on (`blend`, `obj`) whose ST0 answer sets the scale
///    `1 - answer`.
/// 2. Runs a solve callee with two out-pointers (one into dead stack), feeds
///    the first out-word and `this + 0x21c` through a sine-style callee, and
///    wraps the difference of the two answers into [-pi, pi] by repeated
///    2pi steps (an unordered difference exits both loops at once).
/// 3. Blends `this + 0x21c` towards the second answer by the wrapped
///    difference times the scale, and optionally notifies through one more
///    callee when the flag byte at `this + 0x1a8` says so.
/// 4. Refreshes the pose quad at `this + 0x290` from the `0x160` or `0x150`
///    block, or relaxes it towards the `0x150` block with factor 0.1 when the
///    mode word at `this + 0x130` is non-zero (writing the result back to
///    both quads).
/// 5. Converts two small integer answers keyed off `aux` to a float factor
///    (zero when `aux` is null, the second answer only when the byte at
///    `aux + 0x3289` is set), folds it with `this + 0x60` through three
///    constants into `this + 0x220`/`0x218`, and finishes with a commit
///    callee whose answer is also the function's EAX return.
///
/// Original: 0x00a248a0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00a248a0(this: u32, blend: u32, obj: u32, rate: u32, aux: u32) -> u32 {
    unsafe {
        const ONE: f32 = f32::from_bits(0x3f80_0000);
        const PI: f32 = f32::from_bits(0x4049_0fdb);
        const TWO_PI: f32 = f32::from_bits(0x40c9_0fdb);
        const NEG_PI: f32 = f32::from_bits(0xc049_0fdb);
        const RELAX: f32 = f32::from_bits(0x3dcc_cccd); // 0.1
        const K_AUX: f32 = f32::from_bits(0x3c23_d70a); // 0.01
        const K_B: f32 = f32::from_bits(0x3c9f_49f5); // ~0.01944
        const K_C: f32 = f32::from_bits(0x3d2f_8af9); // ~0.04286
        const K_D: f32 = f32::from_bits(0x3f40_0000); // 0.75
        const K_E: f32 = f32::from_bits(0x3e80_0000); // 0.25
        const LOOKUP: u32 = 1;
        const MEASURE: u32 = 2;
        const SOLVE: u32 = 3;
        const SINE: u32 = 4;
        const NOTIFY: u32 = 5;
        const KEY: u32 = 6;
        const KEY_A: u32 = 7;
        const KEY_B: u32 = 8;
        const COMMIT: u32 = 9;
        const FLAG_BIT: u8 = 0x04;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let base = this;
        lf_checker_rt::callee_thiscall!(LOOKUP, u32, obj, base.wrapping_add(0x150), 0x4b3, 1);
        wrf(
            base.wrapping_add(0x158),
            add(rdf(base.wrapping_add(0x158)), f32::from_bits(rate)),
        );
        let measured: f32 = lf_checker_rt::callee_thiscall!(MEASURE, f32, base, blend, obj);
        let scale = sub(ONE, measured);
        let mut scratch: u32 = 0;
        let mut solved: u32 = 0;
        lf_checker_rt::callee_thiscall!(
            SOLVE,
            u32,
            base,
            core::ptr::addr_of_mut!(scratch) as u32,
            core::ptr::addr_of_mut!(solved) as u32
        );
        let first: f32 = lf_checker_rt::callee_cdecl!(SINE, f32, solved);
        let second: f32 = lf_checker_rt::callee_cdecl!(SINE, f32, rd32(base.wrapping_add(0x21c)));
        wrf(base.wrapping_add(0x21c), second);
        let mut head = first;
        while sub(head, second) > PI {
            head = sub(head, TWO_PI);
        }
        while sub(head, second) < NEG_PI {
            head = add(head, TWO_PI);
        }
        let blended = add(mul(sub(head, second), scale), second);
        wrf(base.wrapping_add(0x21c), blended);
        if rd8(base.wrapping_add(0x1a8)) & FLAG_BIT != 0 {
            let target = rd32(base.wrapping_add(0x118));
            if target != 0 {
                lf_checker_rt::callee_thiscall!(NOTIFY, u32, target, target.wrapping_add(0x10), 0x1f4, 1, 1);
            }
        }
        if rd32(base.wrapping_add(0x130)) == 0 {
            let src = if rd8(base.wrapping_add(0x1a8)) & FLAG_BIT != 0 { 0x160 } else { 0x150 };
            wr32(base.wrapping_add(0x290), rd32(base.wrapping_add(src)));
            wr32(base.wrapping_add(0x294), rd32(base.wrapping_add(src + 4)));
            wr32(base.wrapping_add(0x298), rd32(base.wrapping_add(src + 8)));
            wr32(base.wrapping_add(0x29c), rd32(base.wrapping_add(src + 12)));
        } else {
            let p4 = rdf(base.wrapping_add(0x290));
            let d1 = sub(rdf(base.wrapping_add(0x150)), p4);
            let d2 = sub(rdf(base.wrapping_add(0x154)), rdf(base.wrapping_add(0x294)));
            let d3 = sub(rdf(base.wrapping_add(0x158)), rdf(base.wrapping_add(0x298)));
            let e1 = mul(d1, RELAX);
            let e2 = mul(d2, RELAX);
            let e3 = mul(d3, RELAX);
            let n4 = add(p4, e1);
            wrf(base.wrapping_add(0x290), n4);
            let n5 = add(rdf(base.wrapping_add(0x294)), e2);
            let n6 = add(rdf(base.wrapping_add(0x298)), e3);
            wrf(base.wrapping_add(0x294), n5);
            wrf(base.wrapping_add(0x298), n6);
            wr32(base.wrapping_add(0x150), rd32(base.wrapping_add(0x290)));
            wrf(base.wrapping_add(0x154), n5);
            wrf(base.wrapping_add(0x158), n6);
            wr32(base.wrapping_add(0x15c), rd32(base.wrapping_add(0x29c)));
        }
        let mut factor = 0.0f32;
        if aux != 0 {
            let k1: u32 = lf_checker_rt::callee_thiscall!(KEY, u32, aux);
            let a1: u32 = lf_checker_rt::callee_cdecl!(KEY_A, u32, k1);
            factor = (a1 as i32).wrapping_neg() as f32;
            if rd8(aux.wrapping_add(0x3289)) != 0 {
                let k2: u32 = lf_checker_rt::callee_thiscall!(KEY, u32, aux);
                let a2: u32 = lf_checker_rt::callee_cdecl!(KEY_B, u32, k2, 1);
                factor = (a2 as i32).wrapping_neg() as f32;
            }
        }
        let span = rdf(base.wrapping_add(0x60));
        let f = mul(factor, K_AUX);
        let mut g = mul(span, K_B);
        g = mul(g, K_C);
        g = mul(g, f);
        let h = mul(rdf(base.wrapping_add(0x220)), K_D);
        g = mul(g, K_E);
        let total = add(g, h);
        wrf(base.wrapping_add(0x220), total);
        wrf(base.wrapping_add(0x218), add(total, rdf(base.wrapping_add(0x218))));
        let vec = rd32(base.wrapping_add(0x204));
        lf_checker_rt::callee_thiscall!(COMMIT, u32, base, rd32(vec.wrapping_add(0x2b0)), 0, rd32(base.wrapping_add(0x60)))
    }
});
