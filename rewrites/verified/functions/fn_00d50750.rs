// original: 0x00D50750 TransformVectorApply
/// Combines two caller-provided vectors and composes the result into a
/// twelve-word transform. The source object supplies its 3-by-3 basis and
/// translation; an interface object returns the two three-float vectors
/// through vtable slots `+0x60` and `+0x64`. Each vector sum is multiplied by
/// the executable's `-0.5` constant. The resulting X, Y and Z components are
/// accumulated against the destination basis in the original SSE operand
/// order, then added to the copied translation. This proof uses distinct
/// source and destination objects; aliasing is not covered.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00d50750(
    source: u32,
    interface: u32,
    destination: u32,
) {
    unsafe {
        const VECTOR_X: u32 = 0;
        const VECTOR_Y: u32 = 4;
        const VECTOR_Z: u32 = 8;
        const HALF_SCALE_VA: u32 = 0x00fe_8d7c;
        const GET_FIRST_VECTOR: u32 = 1;
        const GET_SECOND_VECTOR: u32 = 2;
        const SOURCE_TO_DESTINATION: [(u32, u32); 12] = [
            (0x00, 0x10),
            (0x04, 0x14),
            (0x08, 0x18),
            (0x10, 0x20),
            (0x14, 0x24),
            (0x18, 0x28),
            (0x20, 0x30),
            (0x24, 0x34),
            (0x28, 0x38),
            (0x30, 0x40),
            (0x34, 0x44),
            (0x38, 0x48),
        ];

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, word: u32) {
            unsafe { (address as *mut u32).write_unaligned(word) }
        }
        #[inline(always)]
        unsafe fn read_float(address: u32) -> f32 {
            f32::from_bits(unsafe { read_word(address) })
        }
        #[inline(always)]
        unsafe fn write_float(address: u32, value: f32) {
            unsafe { write_word(address, value.to_bits()) }
        }
        #[inline(always)]
        fn ordered_add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)]
        fn ordered_mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        let first_vector =
            lf_checker_rt::callee_thiscall!(GET_FIRST_VECTOR, u32, interface);
        let first_x = read_float(first_vector + VECTOR_X);
        let first_y = read_float(first_vector + VECTOR_Y);
        let first_z = read_float(first_vector + VECTOR_Z);
        let second_vector =
            lf_checker_rt::callee_thiscall!(GET_SECOND_VECTOR, u32, interface);
        let second_x = read_float(second_vector + VECTOR_X);
        let second_y = read_float(second_vector + VECTOR_Y);
        let second_z = read_float(second_vector + VECTOR_Z);

        for (destination_offset, source_offset) in SOURCE_TO_DESTINATION {
            write_word(
                destination + destination_offset,
                read_word(source + source_offset),
            );
        }

        let half_scale = lf_checker_rt::global::<f32>(HALF_SCALE_VA).read_unaligned();
        let x_scale = ordered_mul(ordered_add(second_x, first_x), half_scale);
        let y_scale = ordered_mul(ordered_add(second_y, first_y), half_scale);
        let z_scale = ordered_mul(ordered_add(second_z, first_z), half_scale);

        // The original adds the Y product to the X product, then adds Z.
        let translation_x = ordered_add(
            ordered_add(
                ordered_mul(read_float(destination + 0x10), y_scale),
                ordered_mul(read_float(destination), x_scale),
            ),
            ordered_mul(read_float(destination + 0x20), z_scale),
        );
        let translation_y = ordered_add(
            ordered_add(
                ordered_mul(read_float(destination + 0x14), y_scale),
                ordered_mul(read_float(destination + 0x04), x_scale),
            ),
            ordered_mul(read_float(destination + 0x24), z_scale),
        );
        let translation_z = ordered_add(
            ordered_add(
                ordered_mul(read_float(destination + 0x18), y_scale),
                ordered_mul(read_float(destination + 0x08), x_scale),
            ),
            ordered_mul(read_float(destination + 0x28), z_scale),
        );

        write_float(
            destination + 0x30,
            ordered_add(read_float(destination + 0x30), translation_x),
        );
        write_float(
            destination + 0x34,
            ordered_add(read_float(destination + 0x34), translation_y),
        );
        write_float(
            destination + 0x38,
            ordered_add(read_float(destination + 0x38), translation_z),
        );
    }
}
