// original: 0x00b3a010 task_random_pick (proposed)

/// Draw a task-related random pick: fetch the two scaler floats through the
/// scaler callee into scratch slots (the first slot aliases the incoming
/// argument word, which the contract's stack check therefore ignores),
/// multiply the pool count (plus five when the `variant` byte is nonzero) by
/// the selected scaler, and truncate toward zero with x86 convert semantics
/// (out-of-range and NaN yield 0x80000000). Keep the drawn value when it is
/// ordered-below the limiter callee's answer (signed), else re-draw once.
/// Finally return the excess over the floor global, or 0 when at or below
/// the floor. Original: 0x00b3a010 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00b3a010(variant: u32) -> u32 {
    unsafe {
        const SCALER: u32 = 1;
        const LIMITER: u32 = 2;
        const POOL_COUNT: u32 = 0x0169e410;
        const FLOOR: u32 = 0x016624c4;
        const VARIANT_BIAS: u32 = 5;
        /// Truncate a float toward zero with x86 `cvttss2si` semantics.
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                return i32::MIN;
            }
            x as i32
        }
        let mut pool = lf_checker_rt::global::<u32>(POOL_COUNT).read() as i32;
        if variant & 0xff != 0 {
            pool = pool.wrapping_add(VARIANT_BIAS as i32);
        }
        let mut slot_first = 0u32;
        let mut slot_second = 0u32;
        lf_checker_rt::callee_cdecl!(
            SCALER,
            u32,
            core::ptr::addr_of_mut!(slot_first) as u32,
            core::ptr::addr_of_mut!(slot_second) as u32
        );
        let scaler = if variant & 0xff != 0 {
            f32::from_bits(slot_first)
        } else {
            f32::from_bits(slot_second)
        };
        let mut picked =
            cvtt(core::hint::black_box(pool as f32) * core::hint::black_box(scaler));
        let limit: u32 = lf_checker_rt::callee_cdecl!(LIMITER, u32);
        if !(picked < limit as i32) {
            picked = lf_checker_rt::callee_cdecl!(LIMITER, u32) as i32;
        }
        let floor = lf_checker_rt::global::<u32>(FLOOR).read() as i32;
        if floor < picked {
            (picked.wrapping_sub(floor)) as u32
        } else {
            0
        }
    }
});
