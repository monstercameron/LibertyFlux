// original: 0x00CF48B0 climb_task_world_position

/// Copy a position from the receiver and, for placement kinds 2 through 4,
/// transform its three coordinates by the placement's row-major affine
/// matrix. The receiver stores the source vector at `+0x20`; its placement
/// pointer is at `+0x64`. A placement stores its matrix pointer at `+0x20`
/// and its kind in bits 6 through 9 of the word at `+0x28`. The output is a
/// four-word vector: the first three words are copied or transformed, and
/// the fourth is read from an uninitialized local stack word. The contract
/// pins the below-ESP scratch fill to zero and the rewrite mirrors that value;
/// arbitrary scratch residue is outside this proof. This is a thiscall
/// method with one caller-provided output pointer and returns that pointer.
lf_checker_rt::export!(thiscall, rw_00cf48b0(this: u32, output: u32) -> u32 {
    unsafe {
        const SOURCE_VECTOR: u32 = 0x20;
        const PLACEMENT: u32 = 0x64;
        const PLACEMENT_MATRIX: u32 = 0x20;
        const PLACEMENT_KIND: u32 = 0x28;
        const PLACEMENT_MATRIX_FIELD: u32 = 0x10;
        const KIND_SHIFT: u32 = 6;
        const FIRST_TRANSFORM_KIND: u32 = 2;
        const LAST_TRANSFORM_KIND: u32 = 4;
        const OUTPUT_X: u32 = 0;
        const OUTPUT_Y: u32 = 4;
        const OUTPUT_Z: u32 = 8;
        const OUTPUT_W: u32 = 12;
        const MATRIX_STRIDE: u32 = 4;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
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
        fn multiply(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        #[inline(always)]
        fn add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        let source_x = unsafe { read_word(this + SOURCE_VECTOR) };
        let source_y = unsafe { read_word(this + SOURCE_VECTOR + 4) };
        let source_z = unsafe { read_word(this + SOURCE_VECTOR + 8) };
        unsafe {
            write_word(output + OUTPUT_X, source_x);
            write_word(output + OUTPUT_Y, source_y);
            write_word(output + OUTPUT_Z, source_z);
        }

        let placement = unsafe { read_word(this + PLACEMENT) };
        if placement == 0 {
            return output;
        }

        let kind = (unsafe { read_word(placement + PLACEMENT_KIND) } >> KIND_SHIFT) & 0x0f;
        if kind < FIRST_TRANSFORM_KIND || kind > LAST_TRANSFORM_KIND {
            return output;
        }

        if unsafe { read_word(placement + PLACEMENT_MATRIX) } == 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, placement);
            let matrix_after_allocation = unsafe { read_word(placement + PLACEMENT_MATRIX) };
            let _ = lf_checker_rt::callee_thiscall!(
                2,
                u32,
                placement + PLACEMENT_MATRIX_FIELD,
                matrix_after_allocation
            );
        }

        let matrix = unsafe { read_word(placement + PLACEMENT_MATRIX) };
        let x = f32::from_bits(source_x);
        let y = f32::from_bits(source_y);
        let z = f32::from_bits(source_z);

        // Preserve each scalar multiply/add in the original operand order.
        let mut transformed_x = multiply(unsafe { read_float(matrix + 4 * MATRIX_STRIDE) }, y);
        transformed_x = add(
            transformed_x,
            multiply(unsafe { read_float(matrix) }, x),
        );
        transformed_x = add(
            transformed_x,
            multiply(unsafe { read_float(matrix + 8 * MATRIX_STRIDE) }, z),
        );
        transformed_x = add(
            transformed_x,
            unsafe { read_float(matrix + 12 * MATRIX_STRIDE) },
        );

        let mut transformed_y = multiply(unsafe { read_float(matrix + 5 * MATRIX_STRIDE) }, y);
        transformed_y = add(
            transformed_y,
            multiply(unsafe { read_float(matrix + MATRIX_STRIDE) }, x),
        );
        transformed_y = add(
            transformed_y,
            multiply(unsafe { read_float(matrix + 9 * MATRIX_STRIDE) }, z),
        );
        transformed_y = add(
            transformed_y,
            unsafe { read_float(matrix + 13 * MATRIX_STRIDE) },
        );

        let mut transformed_z = multiply(unsafe { read_float(matrix + 6 * MATRIX_STRIDE) }, y);
        transformed_z = add(
            transformed_z,
            multiply(unsafe { read_float(matrix + 2 * MATRIX_STRIDE) }, x),
        );
        transformed_z = add(
            transformed_z,
            multiply(unsafe { read_float(matrix + 10 * MATRIX_STRIDE) }, z),
        );
        transformed_z = add(
            transformed_z,
            unsafe { read_float(matrix + 14 * MATRIX_STRIDE) },
        );

        unsafe {
            write_float(output + OUTPUT_X, transformed_x);
            write_float(output + OUTPUT_Y, transformed_y);
            write_float(output + OUTPUT_Z, transformed_z);
            // The original copies a scratch dword from its local frame here.
            write_word(output + OUTPUT_W, 0);
        }
        output
    }
});
