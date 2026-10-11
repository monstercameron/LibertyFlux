// original: 0x00CF72B0 climb_task_write_transformed_offset
/// Computes a transformed three-dimensional offset for the task's target
/// point. The second stack argument points to a record whose matrix begins at
/// byte offset 0x20. A helper converts the final three scalar arguments into
/// a four-word vector; the matrix rows are accumulated in the source order,
/// each position component is reduced by the corresponding target component,
/// and the helper's fourth word is copied unchanged. The result occupies the
/// task's four-float block at byte offset 0x40 and sets flag bit 1.
lf_checker_rt::export!(thiscall, rw_00cf72b0(
    task: u32,
    target_point: u32,
    transform_record: u32,
    parameter_a: u32,
    parameter_b: u32,
    parameter_c_bits: u32,
) -> u32 {
    const VECTOR_HELPER: u32 = 1;
    const UPDATE_HELPER: u32 = 2;
    const TRANSFORM_MATRIX: u32 = 0x20;
    const TASK_RESULT: u32 = 0x40;
    const TASK_FLAGS: u32 = 0x89;
    const OUTPUT_COMPONENTS: u32 = 4;
    const MATRIX_ROW_STRIDE: u32 = 0x10;

    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe fn read_float(address: u32) -> f32 {
        f32::from_bits(unsafe { read_word(address) })
    }
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }
    #[inline(never)]
    fn multiply(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(never)]
    fn add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    #[inline(never)]
    fn subtract(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }

    let mut helper_output = core::mem::MaybeUninit::<[u32; OUTPUT_COMPONENTS as usize]>::uninit();
    let output_address = helper_output.as_mut_ptr() as *mut u32 as u32;
    let _: u32 = unsafe {
        lf_checker_rt::callee_cdecl!(
            VECTOR_HELPER,
            u32,
            output_address,
            parameter_c_bits,
            parameter_a,
            parameter_b,
        )
    };
    let output_words = unsafe { helper_output.assume_init() };

    unsafe {
        let matrix = read_word(transform_record.wrapping_add(TRANSFORM_MATRIX));
        let v0 = f32::from_bits(output_words[0]);
        let v1 = f32::from_bits(output_words[1]);
        let v2 = f32::from_bits(output_words[2]);

        let x = subtract(
            add(
                add(
                    add(
                        multiply(read_float(matrix + MATRIX_ROW_STRIDE), v1),
                        multiply(read_float(matrix), v0),
                    ),
                    multiply(read_float(matrix + 2 * MATRIX_ROW_STRIDE), v2),
                ),
                read_float(matrix + 3 * MATRIX_ROW_STRIDE),
            ),
            read_float(target_point),
        );
        let y = subtract(
            add(
                add(
                    add(
                        multiply(read_float(matrix + MATRIX_ROW_STRIDE + 4), v1),
                        multiply(read_float(matrix + 4), v0),
                    ),
                    multiply(read_float(matrix + 2 * MATRIX_ROW_STRIDE + 4), v2),
                ),
                read_float(matrix + 3 * MATRIX_ROW_STRIDE + 4),
            ),
            read_float(target_point + 4),
        );
        let z = subtract(
            add(
                add(
                    add(
                        multiply(read_float(matrix + MATRIX_ROW_STRIDE + 8), v1),
                        multiply(read_float(matrix + 8), v0),
                    ),
                    multiply(read_float(matrix + 2 * MATRIX_ROW_STRIDE + 8), v2),
                ),
                read_float(matrix + 3 * MATRIX_ROW_STRIDE + 8),
            ),
            read_float(target_point + 8),
        );

        write_word(task.wrapping_add(TASK_RESULT), x.to_bits());
        write_word(task.wrapping_add(TASK_RESULT + 4), y.to_bits());
        write_word(task.wrapping_add(TASK_RESULT + 8), z.to_bits());
        write_word(task.wrapping_add(TASK_RESULT + 12), output_words[3]);
        let flags = (task.wrapping_add(TASK_FLAGS) as *const u8).read();
        (task.wrapping_add(TASK_FLAGS) as *mut u8).write(flags | 2);
        let _: u32 = lf_checker_rt::callee_thiscall!(UPDATE_HELPER, u32, task);
    }
    0
});
