// original: 0x00B38DF0 population_spawn_update

/// Update the population spawn cycle, prepare three sampled position vectors,
/// and dispatch the scenario work when cutscenes are not holding the gate.
///
/// The cdecl entry takes two pointers, six single-precision values, a mode
/// word, a preferred count and a fallback count. The first pointer supplies a
/// three-float base vector. The fifth float is scaled by two values loaded from
/// the image. A signed counter increment wraps to zero once it reaches twelve.
/// The routine writes the counter, a byte gate, a float accumulator and a
/// request limit. It calls the vector helper, frame update and streaming range
/// helper three times, shuffles the spawn list, then checks the cutscene gate.
/// When the preferred count is zero it uses the fallback count and mode four;
/// otherwise it forwards the preferred count and mode zero. The scenario
/// helper's EAX result is returned, or zero when no work is requested. A true
/// cutscene result returns zero early and leaves the byte gate set.
///
/// Stack-local vector pointers are forwarded to helpers; their contents are
/// the observable payload for those calls. The incoming counts use signed
/// comparisons exactly where the original does.
lf_checker_rt::export!(cdecl, rw_00b38df0(
    base_vector: u32,
    scenario_context: u32,
    x_min: u32,
    y_min: u32,
    z_min: u32,
    scale: u32,
    x_max: u32,
    y_max: u32,
    mode_flags: u32,
    preferred_count: u32,
    fallback_count: u32,
) -> u32 {
    unsafe {
        const GLOBAL_SPAWN_INDEX: u32 = 0x0166_24b4;
        const GLOBAL_SPAWN_GATE: u32 = 0x0166_247a;
        const GLOBAL_ACTIVE_COUNT: u32 = 0x0166_24b0;
        const GLOBAL_ACCUMULATOR: u32 = 0x0166_24b8;
        const GLOBAL_STREAM_LIMIT: u32 = 0x0104_59b4;
        const GLOBAL_STREAM_BIAS: u32 = 0x0104_59bc;
        const SPAWN_INDEX_LIMIT: i32 = 12;
        const STREAM_LIMIT: u32 = 0xc8;
        const SCALE_THIRD: u32 = 0x00fe_8800;
        const SCALE_TWO_THIRDS: u32 = 0x00fe_8870;

        #[inline(always)]
        fn ordered_add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        #[inline(always)]
        fn ordered_mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        #[inline(always)]
        unsafe fn read_f32(file_va: u32) -> f32 {
            unsafe { lf_checker_rt::global::<f32>(file_va).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(file_va: u32, value: u32) {
            unsafe { lf_checker_rt::global::<u32>(file_va).write_unaligned(value) }
        }

        let next_index = lf_checker_rt::global::<u32>(GLOBAL_SPAWN_INDEX)
            .read_unaligned()
            .wrapping_add(1);
        let spawn_index = if (next_index as i32) >= SPAWN_INDEX_LIMIT {
            0
        } else {
            next_index
        };
        write_u32(GLOBAL_SPAWN_INDEX, spawn_index);

        let scale_value = f32::from_bits(scale);
        let scale_third = ordered_mul(scale_value, read_f32(SCALE_THIRD));
        let scale_two_thirds = ordered_mul(scale_value, read_f32(SCALE_TWO_THIRDS));
        let accumulator = ordered_add(
            read_f32(GLOBAL_STREAM_BIAS),
            read_f32(GLOBAL_ACCUMULATOR),
        );

        lf_checker_rt::global::<u8>(GLOBAL_SPAWN_GATE).write(1);
        write_u32(GLOBAL_ACTIVE_COUNT, 0);
        write_u32(GLOBAL_STREAM_LIMIT, STREAM_LIMIT);
        lf_checker_rt::global::<f32>(GLOBAL_ACCUMULATOR).write_unaligned(accumulator);

        let mut vector_out = [0.0f32; 3];
        let mut scalar_out = 0.0f32;
        let vector_out_ptr = vector_out.as_mut_ptr() as usize as u32;
        let scalar_out_ptr = (&mut scalar_out as *mut f32) as usize as u32;

        let _ = lf_checker_rt::callee_cdecl!(
            1,
            u32,
            base_vector,
            scale_two_thirds.to_bits(),
            scale_value.to_bits(),
            vector_out_ptr,
            scalar_out_ptr,
        );
        let _ = lf_checker_rt::callee_cdecl!(
            2,
            u32,
            vector_out_ptr,
            scalar_out.to_bits(),
        );
        let _ = lf_checker_rt::callee_cdecl!(
            3,
            f32,
            vector_out_ptr,
            scalar_out.to_bits(),
        );

        let _ = lf_checker_rt::callee_cdecl!(
            4,
            u32,
            base_vector,
            scale_third.to_bits(),
            scale_two_thirds.to_bits(),
            vector_out_ptr,
            scalar_out_ptr,
        );
        let _ = lf_checker_rt::callee_cdecl!(
            5,
            u32,
            vector_out_ptr,
            scalar_out.to_bits(),
        );
        let _ = lf_checker_rt::callee_cdecl!(
            6,
            f32,
            vector_out_ptr,
            scalar_out.to_bits(),
        );

        let _ = lf_checker_rt::callee_cdecl!(
            7,
            u32,
            base_vector,
            0,
            scale_third.to_bits(),
            vector_out_ptr,
            scalar_out_ptr,
        );
        let _ = lf_checker_rt::callee_cdecl!(
            8,
            u32,
            vector_out_ptr,
            scalar_out.to_bits(),
        );
        let _ = lf_checker_rt::callee_cdecl!(
            9,
            f32,
            vector_out_ptr,
            scalar_out.to_bits(),
        );

        let _ = lf_checker_rt::callee_cdecl!(10, u32,);

        let cutscene_active = lf_checker_rt::callee_cdecl!(11, u32,) as u8 != 0;
        if cutscene_active {
            return 0;
        }

        let (scenario_count, scenario_mode) = if preferred_count != 0 {
            (preferred_count, 0)
        } else {
            (fallback_count, 4)
        };
        let scenario_result = if (scenario_count as i32) > 0 {
            lf_checker_rt::callee_cdecl!(
                12,
                u32,
                base_vector,
                scenario_context,
                x_min,
                y_min,
                z_min,
                scale,
                x_max,
                y_max,
                mode_flags,
                scenario_count,
                scenario_mode,
            )
        } else {
            0
        };

        let _ = lf_checker_rt::callee_cdecl!(13, u32,);
        lf_checker_rt::global::<u8>(GLOBAL_SPAWN_GATE).write(0);
        scenario_result
    }
});
