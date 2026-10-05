// original: 0x00ab5350 stream_process_slot (proposed)

/// Process one streaming timeslot and report whether it fired.
///
/// Forms `t = (*clock + 1.15) - a2` and notifies the watcher callee of
/// `(a1, a2)`. When the gate float `a6` sits strictly inside `(-0.15,
/// 1.15)` the slot fires (`bl = 1`), running the rush callee with `(a0,
/// t, entity, -1, -1)` unless the policy callee vetoes. When the gate's low
/// byte is also nonzero, bills `rate * count` (rate from the rate callee's
/// x87 slot, count from the billing callee) onto `*clock`. Returns `bl`.
/// The float operation order is the original's.
///
/// Callees: 1 = watcher (cdecl, two words), 2 = policy veto (cdecl, no
/// words), 3 = rush (cdecl, five words), 4 = billing (cdecl, four words),
/// 5 = rate (cdecl, no words, float in ST0).
///
/// Original: 0x00ab5350 (cdecl, seven stack words).
lf_checker_rt::export!(cdecl, rw_00ab5350(
    a0: u32,
    a1: u32,
    a2: u32,
    _dead: u32,
    clock: u32,
    entity: u32,
    gate: u32,
) -> u32 {
    unsafe {
        const WATCHER: u32 = 1;
        const POLICY: u32 = 2;
        const RUSH: u32 = 3;
        const BILLING: u32 = 4;
        const RATE: u32 = 5;
        const WINDOW: f32 = f32::from_bits(0x3F93_3333); // 1.15
        const FLOOR: f32 = f32::from_bits(0xBE19_999A); // -0.15
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
        let old = f32::from_bits((clock as *const u32).read_unaligned());
        let t = sub(add(old, WINDOW), f32::from_bits(a2));
        lf_checker_rt::callee_cdecl!(WATCHER, u32, a1, a2);
        let g = f32::from_bits(gate);
        let mut fired = 0u32;
        if g > FLOOR && WINDOW > g {
            fired = 1;
            if lf_checker_rt::callee_cdecl!(POLICY, u32,) & 0xFF == 0 {
                lf_checker_rt::callee_cdecl!(
                    RUSH, u32, a0, t.to_bits(), entity, 0xFFFF_FFFF, 0xFFFF_FFFF
                );
            }
        }
        if (gate & 0xFF) != 0 {
            let n = lf_checker_rt::callee_cdecl!(BILLING, u32, a2, t.to_bits(), entity, 0);
            let rate: f32 = lf_checker_rt::callee_cdecl!(RATE, f32,);
            let billed = add(mul(rate, n as f32), old);
            (clock as *mut u32).write_unaligned(billed.to_bits());
        }
        fired
    }
});
