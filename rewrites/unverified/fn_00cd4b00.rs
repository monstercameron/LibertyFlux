// original: 0x00CD4B00 peds_task_steer_blend2
/// Updates a task's steering blend and its child blend state.
///
/// The task flag gates two integer-derived steering components. A low object
/// type nibble enables the normalized magnitude path, which calls the pool,
/// double-precision blend helper, and trigonometric helpers before using a
/// scripted gate. A second flag-gated state check selects the coefficient
/// ladder. The result is clamped, reflected in the task flag, stored through
/// the child helpers, and returned from the child's virtual completion call.
/// Float operations retain the source order, and the double helper's two
/// inputs and answer are carried as their full bit patterns.
lf_checker_rt::export!(thiscall, rw_00CD4B00(this: u32, a0: u32) -> u32 {
    unsafe {
        const CHILD_OFF: u32 = 0x0a80;
        const FLAG_OFF: u32 = 0x68;
        const TYPE_OFF: u32 = 0x1e2;
        const DETAIL_OFF: u32 = 0x228;
        const DETAIL_STATUS_OFF: u32 = 0x484;
        const CHILD_VT_SLOT: u32 = 0x4c;
        const POOL: u32 = 0x0128e310;
        const G_DIVISOR: u32 = 0x01051954;
        const G_LOW: u32 = 0x01051958;
        const G_MIDDLE: u32 = 0x0105195c;
        const G_HIGH: u32 = 0x01051960;
        const G_DETAIL_GATE: u32 = 0x017f5ea0;
        const G_DETAIL_ENABLE: u32 = 0x0103ce4a;
        const F_ONE: u32 = 0x00fe88e8;
        const F_DETAIL_THRESHOLD: u32 = 0x00fe8830;
        const F_OUTPUT_LIMIT: u32 = 0x00fe8a94;
        const SIGN_MASK: u32 = 0x80000000;

        #[inline(always)]
        unsafe fn rd32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(addr: u32, value: u32) {
            unsafe { (addr as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn rd8(addr: u32) -> u8 {
            unsafe { (addr as *const u8).read() }
        }
        #[inline(always)]
        fn global_f32(addr: u32) -> f32 {
            f32::from_bits(unsafe {
                (lf_checker_rt::global::<u32>(addr) as *const u32).read_unaligned()
            })
        }
        #[inline(always)]
        fn global_u8(addr: u32) -> u8 {
            unsafe { (lf_checker_rt::global::<u8>(addr) as *const u8).read() }
        }
        #[inline(always)]
        fn mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        fn add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)]
        fn div(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) / core::hint::black_box(right)
        }
        #[inline(always)]
        fn negate(value: f32) -> f32 {
            f32::from_bits(value.to_bits() ^ SIGN_MASK)
        }
        #[inline(always)]
        fn scaled_bound(bound: f32, factor: f32) -> f32 {
            let scaled = mul(bound, factor);
            if bound > scaled { scaled } else { bound }
        }

        let child = rd32(a0 + CHILD_OFF);
        let _: u32 = lf_checker_rt::callee_thiscall!(0, u32, child, 0);
        let probe: u32 = lf_checker_rt::callee_thiscall!(1, u32, a0);

        let initial_flags = rd8(this + FLAG_OFF);
        let mut component_x = 0.0f32;
        let mut component_y = 0.0f32;
        if initial_flags & 4 != 0 {
            let first_probe: u32 = lf_checker_rt::callee_thiscall!(2, u32, probe);
            let first_integer: u32 = lf_checker_rt::callee_cdecl!(3, u32, first_probe);
            component_x = (first_integer as i32) as f32;

            let second_probe: u32 = lf_checker_rt::callee_thiscall!(4, u32, probe);
            let second_integer: u32 = lf_checker_rt::callee_cdecl!(3, u32, second_probe);
            component_y = negate((second_integer as i32) as f32);
        }

        let mut ratio = 0.0f32;
        let mut main_path_ran = false;
        let mut mix_result_bits = 0u32;
        let object_nibble = rd8(a0 + TYPE_OFF) & 0x0f;
        if initial_flags & 4 != 0 && object_nibble < 2 {
            let x_squared = mul(component_x, component_x);
            let y_squared = mul(component_y, component_y);
            let magnitude = add(x_squared, y_squared).sqrt();
            let mut measured = div(magnitude, global_f32(G_DIVISOR));
            if measured > global_f32(F_ONE) {
                measured = global_f32(F_ONE);
            }
            if measured > 0.0 {
                ratio = measured;
                main_path_ran = true;

                let pool = lf_checker_rt::relocated(POOL);
                let pool_bits: u32 =
                    lf_checker_rt::callee_thiscall!(5, u32, pool, a0, 1);
                let pool_value = f32::from_bits(pool_bits);

                let double_x = core::hint::black_box(negate(component_x)) as f64;
                let double_y = core::hint::black_box(component_y) as f64;
                let x_bits = double_x.to_bits();
                let y_bits = double_y.to_bits();
                let double_answer: u64 = lf_checker_rt::callee_cdecl!(
                    6,
                    u64,
                    x_bits as u32,
                    (x_bits >> 32) as u32,
                    y_bits as u32,
                    (y_bits >> 32) as u32
                );
                let narrowed_answer =
                    core::hint::black_box(f64::from_bits(double_answer)) as f32;
                let combined = add(pool_value, narrowed_answer);

                let mixed_bits: u32 =
                    lf_checker_rt::callee_thiscall!(7, u32, 0, combined.to_bits());
                let mixed_value = f32::from_bits(mixed_bits);
                mix_result_bits = mixed_value.to_bits();
                wr32(a0 + 0x0aa4, mix_result_bits);

                let cosine_bits: u32 =
                    lf_checker_rt::callee_cdecl!(8, u32, mixed_value.to_bits());
                let negative_cosine = negate(f32::from_bits(cosine_bits));
                let sine_bits: u32 =
                    lf_checker_rt::callee_cdecl!(9, u32, mixed_value.to_bits());
                let gate: u32 = lf_checker_rt::callee_cdecl!(
                    10,
                    u32,
                    a0,
                    (&negative_cosine as *const f32) as u32,
                    0,
                    sine_bits
                );
                if gate & 0xff == 0 {
                    ratio = 0.0;
                }
            }
        }

        let mut result_value: f32;
        if initial_flags & 4 == 0 {
            result_value = scaled_bound(global_f32(G_MIDDLE), ratio);
        } else {
            let detail = rd32(a0 + DETAIL_OFF);
            if rd8(detail + DETAIL_STATUS_OFF) != 0 {
                result_value = scaled_bound(global_f32(G_MIDDLE), ratio);
            } else {
                let detail_level_bits: u32 =
                    lf_checker_rt::callee_thiscall!(11, u32, a0, 0, 1);
                let detail_level = f32::from_bits(detail_level_bits);
                let detail_answer: u32 = lf_checker_rt::callee_cdecl!(12, u32,);

                let mut use_special_ladder = true;
                if detail_answer & 0xff == 0
                    && global_u8(G_DETAIL_GATE) == (detail_answer as u8)
                {
                    use_special_ladder = false;
                }
                if use_special_ladder && global_u8(G_DETAIL_ENABLE) != 0 {
                    let detail_ready = rd32(a0 + 0x0b0) != 0
                        && (rd32(a0 + 0x0b8) as i32) > 0;
                    if detail_ready {
                        let ready: u32 =
                            lf_checker_rt::callee_thiscall!(13, u32, a0 + 0x80);
                        if ready & 0xff == 0 {
                            use_special_ladder = false;
                        }
                    }
                }

                if use_special_ladder {
                    if detail_level > global_f32(F_DETAIL_THRESHOLD) {
                        if detail_level > global_f32(F_ONE) {
                            result_value = scaled_bound(global_f32(G_HIGH), ratio);
                        } else {
                            result_value = scaled_bound(global_f32(G_MIDDLE), ratio);
                        }
                    } else {
                        result_value = scaled_bound(global_f32(G_MIDDLE), ratio);
                    }
                } else if detail_level > global_f32(F_ONE) {
                    result_value = scaled_bound(global_f32(G_HIGH), ratio);
                } else if detail_level > 0.0 {
                    result_value = scaled_bound(global_f32(G_MIDDLE), ratio);
                } else {
                    result_value = scaled_bound(global_f32(G_LOW), ratio);
                }
            }
        }

        if result_value < 0.0 {
            result_value = 0.0;
        }
        let output_limit = global_f32(F_OUTPUT_LIMIT);
        let active = if result_value > output_limit {
            result_value = output_limit;
            true
        } else {
            result_value != 0.0
        };
        let flags = rd8(this + FLAG_OFF);
        ((this + FLAG_OFF) as *mut u8).write((flags & !2) | ((active as u8) << 1));

        let _: u32 = lf_checker_rt::callee_thiscall!(14, u32, child, result_value.to_bits(), 0);
        let mut fill_words = [rd32(child + 0x10), rd32(child + 0x0c)];
        let fill_ptr = fill_words.as_mut_ptr() as u32;
        let fill_ecx = if main_path_ran { mix_result_bits } else { 0 };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            15,
            u32,
            fill_ecx,
            a0,
            fill_ptr,
            fill_ptr + 4
        );
        wr32(child + 8, fill_words[0]);
        wr32(child + 4, fill_words[1]);

        let vtable = rd32(child);
        let complete: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable + CHILD_VT_SLOT) as usize);
        complete(child)
    }
});
