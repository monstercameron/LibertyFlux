// original: 0x00a13710 damped_value_update (proposed)
/// Blend a target value toward the stored one, with a snap override.
///
/// When the word at `this + 0x130` is non-zero, the float at `this + 0x174`
/// is moved toward `target` by `stored + (target - stored) * RATE`, where
/// `RATE` is the constant 0x3ea8f5c3, computed in that operand order with
/// single rounding per step. Otherwise the slot is overwritten with `target`
/// unchanged. When `snap`'s low byte is non-zero, the slot is then
/// overwritten with 15.0 (0x41780000). No return value is set. Thiscall,
/// two stack arguments.
export!(thiscall, rw_00a13710(this: u32, snap: u32, target: u32) -> u32 {
    unsafe {
        const ENABLE_OFF: u32 = 0x130;
        const SLOT_OFF: u32 = 0x174;
        const RATE_ADDR: u32 = 0x00fe8800;
        const SNAP_BITS: u32 = 0x41780000;
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
        let goal = f32::from_bits(target);
        if ((this + ENABLE_OFF) as *const u32).read_unaligned() != 0 {
            let stored =
                f32::from_bits(((this + SLOT_OFF) as *const u32).read_unaligned());
            let rate = f32::from_bits(*global::<u32>(RATE_ADDR));
            let next = add(mul(sub(goal, stored), rate), stored);
            ((this + SLOT_OFF) as *mut u32).write_unaligned(next.to_bits());
        } else {
            ((this + SLOT_OFF) as *mut u32).write_unaligned(goal.to_bits());
        }
        if (snap & 0xff) != 0 {
            ((this + SLOT_OFF) as *mut u32).write_unaligned(SNAP_BITS);
        }
        0
    }
});
