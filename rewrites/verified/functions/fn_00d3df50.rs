// original: 0x00D3DF50 peds_task_accumulate_path_distance

/// Update a task's path outputs from the helper's sampled point list.
///
/// `this` object supplies a context pointer from byte offset `+4`; byte `+0`
/// is cleared after the first helper accepts that context, and the
/// state word at `+8` receives two flag bits from the final metadata entry.
/// The five stack arguments are an output status word, an in/out accumulated
/// `f32`, a four-word output vector, an optional four-word previous-vector
/// output, and an optional in/out index word. The index word's initial value
/// is a signed copy limit; the function resets it before calling helpers and
/// later stores the last copied point index there. The status, accumulator,
/// and vector pointers must be valid on paths that write them.
///
/// A rejected context writes status zero and returns false. An accepted
/// context starts with status one and asks a helper for a signed point count,
/// a list of four-word points, and a metadata list. A failed helper or a
/// nonpositive signed count clears the accumulator and the vector's first
/// three words, writes status two, and returns true. For a positive count,
/// the first point seeds the output vector; each point then contributes the
/// distance from the preceding output vector to the accumulator, after which
/// that point becomes the new output vector. When the optional output is
/// present, points whose signed index is below the saved copy limit are
/// copied there and the index word is updated when it is present. Finally, bits zero and one of
/// the task state are replaced by the corresponding bits from the last
/// metadata word. Float operations retain the original operand order and are
/// compared bit for bit. The function is `thiscall` and callee-cleans five
/// stack words; only the low byte of its return register is meaningful.
lf_checker_rt::export!(thiscall, rw_00d3df50(
    this: u32,
    status_ptr: u32,
    accumulator_ptr: u32,
    result_vector_ptr: u32,
    previous_vector_ptr: u32,
    index_ptr: u32,
) -> u32 {
    unsafe {
        const TASK_CONTEXT: u32 = 0x04;
        const TASK_FLAGS: u32 = 0x08;
        const TASK_ACTIVE_BYTE: u32 = 0x00;
        const POINT_STRIDE: u32 = 0x10;
        const META_STRIDE: u32 = 0x04;
        const META_FLAG_SHIFT: u32 = 10;
        const VECTOR_X: u32 = 0x00;
        const VECTOR_Y: u32 = 0x04;
        const VECTOR_Z: u32 = 0x08;
        const VECTOR_W: u32 = 0x0c;
        const CALLEE_ACCEPT_CONTEXT: u32 = 1;
        const CALLEE_GET_POINTS: u32 = 2;
        const CALLEE_FINISH_CONTEXT: u32 = 3;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
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
        unsafe fn write_f32(address: u32, value: f32) {
            unsafe { write_u32(address, value.to_bits()) }
        }
        #[inline(always)]
        unsafe fn copy_vector(source: u32, destination: u32) {
            unsafe {
                for offset in [VECTOR_X, VECTOR_Y, VECTOR_Z, VECTOR_W] {
                    write_u32(destination.wrapping_add(offset), read_u32(source.wrapping_add(offset)));
                }
            }
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
        fn add_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)]
        fn sqrt_f32(value: f32) -> f32 {
            core::hint::black_box(value).sqrt()
        }

        let mut saved_copy_limit = 0u32;
        if index_ptr != 0 {
            saved_copy_limit = read_u32(index_ptr);
            write_u32(index_ptr, 0);
        }

        let task_context = read_u32(this.wrapping_add(TASK_CONTEXT));
        let context_accepted =
            lf_checker_rt::callee_cdecl!(CALLEE_ACCEPT_CONTEXT, u32, task_context);
        if context_accepted as u8 == 0 {
            write_u32(status_ptr, 0);
            return 0;
        }

        write_u32(status_ptr, 1);
        write_u8(this.wrapping_add(TASK_ACTIVE_BYTE), 0);

        let mut helper_count = 0u32;
        let mut point_data = 0u32;
        let mut metadata_data = 0u32;
        let count_out = (&mut helper_count as *mut u32) as u32;
        let point_out = (&mut point_data as *mut u32) as u32;
        let metadata_out = (&mut metadata_data as *mut u32) as u32;
        let helper_status = lf_checker_rt::callee_cdecl!(
            CALLEE_GET_POINTS,
            u32,
            task_context,
            count_out,
            point_out,
            metadata_out,
            0,
        );

        if helper_status != 0 || (helper_count as i32) <= 0 {
            write_u32(accumulator_ptr, 0);
            write_u32(result_vector_ptr.wrapping_add(VECTOR_Z), 0);
            write_u32(result_vector_ptr.wrapping_add(VECTOR_Y), 0);
            write_u32(result_vector_ptr.wrapping_add(VECTOR_X), 0);
            write_u32(status_ptr, 2);
            let _ = lf_checker_rt::callee_cdecl!(
                CALLEE_FINISH_CONTEXT,
                u32,
                task_context,
            );
            return 1;
        }

        write_u32(accumulator_ptr, 0);
        copy_vector(point_data, result_vector_ptr);

        let signed_count = helper_count as i32;
        let signed_copy_limit = saved_copy_limit as i32;
        let mut point_index = 0u32;
        while (point_index as i32) < signed_count {
            let point = point_data.wrapping_add(point_index.wrapping_mul(POINT_STRIDE));
            if previous_vector_ptr != 0 && (point_index as i32) < signed_copy_limit {
                copy_vector(point, previous_vector_ptr);
                write_u32(index_ptr, point_index);
            }

            let dx = sub_f32(read_f32(point.wrapping_add(VECTOR_X)), read_f32(result_vector_ptr.wrapping_add(VECTOR_X)));
            let dy = sub_f32(read_f32(point.wrapping_add(VECTOR_Y)), read_f32(result_vector_ptr.wrapping_add(VECTOR_Y)));
            let dz = sub_f32(read_f32(point.wrapping_add(VECTOR_Z)), read_f32(result_vector_ptr.wrapping_add(VECTOR_Z)));
            let dx_squared = mul_f32(dx, dx);
            let dy_squared = mul_f32(dy, dy);
            let dz_squared = mul_f32(dz, dz);
            let xy_squared = add_f32(dy_squared, dx_squared);
            let xyz_squared = add_f32(xy_squared, dz_squared);
            let distance = sqrt_f32(xyz_squared);
            let accumulated = add_f32(read_f32(accumulator_ptr), distance);
            write_f32(accumulator_ptr, accumulated);

            copy_vector(point, result_vector_ptr);
            point_index = point_index.wrapping_add(1);
        }

        let last_metadata = metadata_data
            .wrapping_add(helper_count.wrapping_mul(META_STRIDE))
            .wrapping_sub(META_STRIDE);
        let last_word = u32::from(read_u16(last_metadata));
        let flags_address = this.wrapping_add(TASK_FLAGS);
        let mut flags = read_u32(flags_address);
        let low_flag = ((last_word >> META_FLAG_SHIFT) ^ flags) & 1;
        flags ^= low_flag;
        let last_metadata_byte = u32::from(read_u8(last_metadata.wrapping_add(1)));
        let second_flag = (last_metadata_byte ^ flags) & 2;
        flags ^= second_flag;
        write_u32(flags_address, flags);

        let _ = lf_checker_rt::callee_cdecl!(
            CALLEE_FINISH_CONTEXT,
            u32,
            task_context,
        );
        1
    }
});
