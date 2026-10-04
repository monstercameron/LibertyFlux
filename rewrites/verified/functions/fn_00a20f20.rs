// original: 0x00a20f20 cam_rate_adapt (proposed)

/// Adapts a stored rate toward 0.5/1.0 or clamps its decayed value.
///
/// `this` points to a record with an unsigned mode word at `+MODE_OFF`
/// and a rate float at `+RATE_OFF`; `p` points to a record with an option
/// word at `+OPT_OFF`. When the option's high bit is set the target is
/// 0.5, otherwise 1.0. At mode 5 or below the rate is set to the target
/// directly. Above mode 5 the rate is first scaled (by 0.99 toward 0.5,
/// by 1.01 toward 1.0) and then clamped: not below 0.5 in the first case,
/// not above 1.0 in the second. A NaN scaled value is stored as is.
/// Returns nothing.
///
/// Original: 0x00a20f20 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a20f20(this: u32, p: u32) -> u32 {
    unsafe {
        const MODE_OFF: u32 = 0x130;
        const RATE_OFF: u32 = 0x32c;
        const OPT_OFF: u32 = 0x24;
        const OPT_BIT: u32 = 0x0800_0000;
        const DECAY_LO: f32 = f32::from_bits(0x3f7d_70a4); // 0.99
        const GROW_HI: f32 = f32::from_bits(0x3f81_47ae); // 1.01
        const TARGET_LO: f32 = f32::from_bits(0x3f00_0000); // 0.5
        const TARGET_HI: f32 = f32::from_bits(0x3f80_0000); // 1.0
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        unsafe fn rd32(x: u32) -> u32 {
            unsafe { (x as *const u32).read_unaligned() }
        }
        unsafe fn set_rate(this: u32, v: f32) {
            unsafe { ((this + RATE_OFF) as *mut u32).write_unaligned(v.to_bits()) }
        }
        let hi_opt = rd32(p + OPT_OFF) & OPT_BIT != 0;
        if rd32(this + MODE_OFF) > 5 {
            let cur = f32::from_bits(rd32(this + RATE_OFF));
            if hi_opt {
                let x0 = mul(cur, DECAY_LO);
                set_rate(this, if TARGET_LO > x0 { TARGET_LO } else { x0 });
            } else {
                let x0 = mul(cur, GROW_HI);
                set_rate(this, if x0 > TARGET_HI { TARGET_HI } else { x0 });
            }
        } else if hi_opt {
            set_rate(this, TARGET_LO);
        } else {
            set_rate(this, TARGET_HI);
        }
        0
    }
});
