// original: 0x00B73690 task_ctor_delta_compare

/// Construct a task, derive a normalized direction value, and store its
/// duration. The object supplies the entity and configuration; the argument
/// points at the host record. Intercepted callees provide the constructor,
/// query, sampling, angle, and random answers. The checker contract defines
/// the uninitialized local used as the initial delta as zero and exercises
/// the branch where the task's mode bit is set.
///
/// The direction path converts the two single-precision components to
/// doubles in the order expected by the angle helper, rounds its result
/// back to a single, wraps the difference into a full-turn interval, and
/// scales its magnitude into the final blend. The task is stored on the
/// object before the duration is written. The final return is the original
/// low-level residue derived from the optional link flags.
fn run_task_ctor_delta_compare(this: u32, host: u32) -> u32 {
    unsafe {
        const ENTITY_PTR: u32 = 0x1c;
        const OWNER_PTR: u32 = 0x34;
        const CREATED_TASK: u32 = 0x18;
        const OWNER_METHOD_OBJECT: u32 = 0x78;
        const MODE: u32 = 0x1304;
        const ENTITY_KIND: u32 = 0x2e;
        const ENTITY_VECTOR: u32 = 0x20;
        const HOST_VECTOR: u32 = 0x20;
        const HOST_FALLBACK_ANGLE: u32 = 0x1c;
        const TASK_FLAGS: u32 = 0x04;
        const TASK_MODE: u32 = 0x40;
        const TASK_MODE_ENABLED: u32 = 0x44;
        const TASK_DURATION: u32 = 0x54;
        const LINK: u32 = 0x6c;
        const VTABLE_ANGLE_QUERY: u32 = 0xa0;
        const VTABLE_DURATION_GATE: u32 = 0x128;
        const ENTITY_TABLE_VA: u32 = 0x1295_cd8;
        const ATTACH_VTABLE_VA: u32 = 0x00b6_fda0;
        const F32_SIGN: u32 = 0x8000_0000;
        const F32_PI: u32 = 0x4049_0fdb;
        const F32_TAU: u32 = 0x40c9_0fdb;
        const F32_NEG_PI: u32 = 0xc049_0fdb;
        const F32_INV_PI: u32 = 0x3ea2_f983;
        const F32_HALF: u32 = 0x3f00_0000;
        const F32_ONE: u32 = 0x3f80_0000;
        const F32_RNG_SCALE: u32 = 0x3800_0100;
        const F32_DURATION_SCALE: u32 = 0x3ecc_ccce;
        const F32_DURATION_BIAS: u32 = 0x3f4c_cccd;
        const F32_DEFAULT_RATE: u32 = 0x4080_0000;
        const F32_FAST_RATE: u32 = 0x447a_0000;
        const F32_SLOW_RATE: u32 = 0x4000_0000;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            unsafe { f32::from_bits(read_u32(address)) }
        }

        #[inline(always)]
        fn add_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        #[inline(always)]
        fn sub_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        #[inline(always)]
        fn mul_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        #[inline(always)]
        fn atan2_from_f32_bits(x_bits: u32, y_bits: u32) -> f32 {
            let x = f32::from_bits(x_bits ^ F32_SIGN);
            let y = f32::from_bits(y_bits);
            let x64 = f64::from(x).to_bits();
            let y64 = f64::from(y).to_bits();
            let result_bits: u64 = lf_checker_rt::callee_cdecl!(
                8,
                u64,
                x64 as u32,
                (x64 >> 32) as u32,
                y64 as u32,
                (y64 >> 32) as u32
            );
            f64::from_bits(result_bits) as f32
        }

        #[inline(always)]
        unsafe fn virtual_thiscall0(this: u32, slot: u32) -> u32 {
            let vtable = unsafe { read_u32(this) };
            let target = unsafe { read_u32(vtable.wrapping_add(slot)) };
            let call: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            call(this)
        }

        let entity = read_u32(this.wrapping_add(ENTITY_PTR));
        let owner_arg = read_u32(this.wrapping_add(OWNER_PTR));
        let component: u32 =
            lf_checker_rt::callee_thiscall!(0, u32, entity, owner_arg);

        let mut ignored_out = 0u32;
        lf_checker_rt::callee_thiscall!(
            1,
            u32,
            component,
            entity,
            &mut ignored_out as *mut u32 as u32
        );
        let state = lf_checker_rt::callee_thiscall!(2, u32, component, entity);

        let mut state_code = match state {
            5 => 0x137,
            6 => 0x139,
            7 => 0x138,
            8 => 0x13a,
            _ => 0,
        };
        let entity_kind = read_u16(entity.wrapping_add(ENTITY_KIND)) as i16 as i32;
        let table_address = lf_checker_rt::relocated(ENTITY_TABLE_VA)
            .wrapping_add((entity_kind as u32).wrapping_mul(4));
        let entity_table = read_u32(table_address);
        let entity_data = read_u32(entity_table.wrapping_add(0xc4));

        let mut ignored_code = 0u32;
        let setup_result: u32 = lf_checker_rt::callee_cdecl!(
            3,
            u32,
            entity_data,
            &mut state_code as *mut u32 as u32,
            host,
            entity,
            0,
            0,
            0,
            1
        );
        ignored_code = setup_result;

        let mode = read_u32(entity.wrapping_add(MODE));
        let rate_bits = match mode {
            3 => F32_FAST_RATE,
            1 => F32_SLOW_RATE,
            _ => F32_DEFAULT_RATE,
        };
        let owner = read_u32(host.wrapping_add(OWNER_METHOD_OBJECT));
        let task: u32 = lf_checker_rt::callee_thiscall!(
            4,
            u32,
            owner,
            ignored_code,
            state_code,
            rate_bits,
            u32::MAX
        );
        write_u32(this.wrapping_add(CREATED_TASK), task);

        lf_checker_rt::callee_thiscall!(
            5,
            u32,
            task,
            1,
            lf_checker_rt::relocated(ATTACH_VTABLE_VA),
            this
        );

        let mut scratch_vector = [0u32; 4];
        let vector_address = scratch_vector.as_mut_ptr() as u32;
        let _setup_vector: u32 = lf_checker_rt::callee_cdecl!(
            6,
            u32,
            vector_address,
            F32_ONE,
            ignored_code,
            vector_address
        );
        let mut scratch_point = [0u32; 4];
        let _sample: u32 = lf_checker_rt::callee_thiscall!(
            7,
            u32,
            scratch_point.as_mut_ptr() as u32,
            scratch_vector.as_mut_ptr() as u32
        );

        let mode_enabled = read_u16(task.wrapping_add(TASK_MODE_ENABLED)) == 1;
        let mode_data = if mode_enabled {
            read_u32(task.wrapping_add(TASK_MODE))
        } else {
            0
        };
        let initial_delta = 0.0f32;
        let mut delta = initial_delta;
        if read_u8(mode_data.wrapping_add(0x28)) & 4 == 0 {
            let first = unsafe { virtual_thiscall0(host, VTABLE_ANGLE_QUERY) };
            let related = if first != 0 {
                let second = unsafe { virtual_thiscall0(host, VTABLE_ANGLE_QUERY) };
                if second != 0 {
                    unsafe { virtual_thiscall0(second, 0xe0) }
                } else {
                    0
                }
            } else {
                read_u32(host.wrapping_add(0x100))
            };
            let sample = read_u32(related.wrapping_add(4));
            let sample = read_u32(sample);
            let sample_value = read_f32(sample.wrapping_add(0x38));
            let sample_bits = sample_value.to_bits();
            let positive_sample = f32::from_bits(sample_bits & !F32_SIGN);
            delta = sub_f32(initial_delta, positive_sample);
        }

        let entity_vector = read_u32(entity.wrapping_add(ENTITY_VECTOR));
        let first_angle = if entity_vector != 0 {
            atan2_from_f32_bits(
                read_u32(entity_vector.wrapping_add(0x10)),
                read_u32(entity_vector.wrapping_add(0x14)),
            )
        } else {
            read_f32(entity.wrapping_add(0x1c))
        };
        let first_angle = add_f32(delta, first_angle);
        let transformed_first_angle = lf_checker_rt::callee_cdecl!(
            9,
            f32,
            first_angle.to_bits()
        );

        let host_vector = read_u32(host.wrapping_add(HOST_VECTOR));
        let calculated_second_angle = if host_vector != 0 {
            atan2_from_f32_bits(
                read_u32(host_vector.wrapping_add(0x10)),
                read_u32(host_vector.wrapping_add(0x14)),
            )
        } else {
            read_f32(host.wrapping_add(HOST_FALLBACK_ANGLE))
        };
        let second_angle = calculated_second_angle;

        let mut difference = sub_f32(transformed_first_angle, second_angle);
        let pi = f32::from_bits(F32_PI);
        let tau = f32::from_bits(F32_TAU);
        let negative_pi = f32::from_bits(F32_NEG_PI);
        while difference > pi {
            difference = sub_f32(difference, tau);
        }
        while difference < negative_pi {
            difference = add_f32(difference, tau);
        }
        if difference < 0.0 {
            difference = f32::from_bits(difference.to_bits() ^ F32_SIGN);
        }

        let normalized = mul_f32(
            mul_f32(difference, f32::from_bits(F32_INV_PI)),
            f32::from_bits(F32_HALF),
        );
        let blend = sub_f32(f32::from_bits(F32_ONE), normalized);
        let gate: u32 = unsafe { virtual_thiscall0(host, VTABLE_DURATION_GATE) };
        let duration = if gate & 0xff != 0 {
            blend
        } else {
            let random: u32 = lf_checker_rt::callee_cdecl!(10, u32,);
            let unit = (random as i32) as f32;
            let random_weight = mul_f32(
                mul_f32(unit, f32::from_bits(F32_RNG_SCALE)),
                f32::from_bits(F32_DURATION_SCALE),
            );
            let biased_weight = add_f32(random_weight, f32::from_bits(F32_DURATION_BIAS));
            mul_f32(biased_weight, blend)
        };
        write_u32(
            task.wrapping_add(TASK_DURATION),
            duration.to_bits(),
        );

        let linked = read_u32(host.wrapping_add(LINK));
        if linked == 0 || read_u8(linked.wrapping_add(0x0e)) == 0 {
            linked
        } else {
            let flags_address = task.wrapping_add(TASK_FLAGS);
            let flags = read_u32(flags_address);
            let returned_flags = flags >> 15;
            if returned_flags & 1 != 0 {
                write_u32(flags_address, flags & !0x8000);
            }
            returned_flags
        }
    }
}

lf_checker_rt::export!(thiscall, rw_00b73690(this: u32, host: u32) -> u32 {
    run_task_ctor_delta_compare(this, host)
});
