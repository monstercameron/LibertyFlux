// original: 0x00A33CB0 matrix_release_to_placement

/// Release this entity's matrix back to the placement manager. If the matrix
/// flag at `+0x40` is clear, the entity's matrix pointer at `+0x20` is simply
/// cleared. Otherwise the routine calls the double-precision angle helper
/// with the sign-flipped float at matrix `+0x10` and the float at `+0x14`,
/// copies matrix floats `+0x30`, `+0x34` and `+0x38` into this entity at
/// `+0x10`, `+0x14` and `+0x18`, stores the returned angle at `+0x1c`, clears
/// both pointers/flags, then releases the matrix node. The angle helper's
/// double arguments and result use XMM registers; its stack words are only the
/// checker's transport. Thiscall receives the entity in ECX.
lf_checker_rt::export!(thiscall, rw_00a33cb0(this: u32) -> () {
    const ENTITY_MATRIX: u32 = 0x20;
    const ENTITY_X: u32 = 0x10;
    const ENTITY_Y: u32 = 0x14;
    const ENTITY_Z: u32 = 0x18;
    const ENTITY_ANGLE: u32 = 0x1c;
    const MATRIX_ANGLE_X: u32 = 0x10;
    const MATRIX_ANGLE_Y: u32 = 0x14;
    const MATRIX_ACTIVE: u32 = 0x40;
    const MATRIX_X: u32 = 0x30;
    const MATRIX_Y: u32 = 0x34;
    const MATRIX_Z: u32 = 0x38;
    const MATRIX_MANAGER_VA: u32 = 0x012d_df60;
    const ATAN2: u32 = 1;
    const RELEASE_NODE: u32 = 2;
    const SIGN_BIT: u32 = 0x8000_0000;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let matrix = read_u32(this.wrapping_add(ENTITY_MATRIX));
        if read_u32(matrix.wrapping_add(MATRIX_ACTIVE)) == 0 {
            write_u32(this.wrapping_add(ENTITY_MATRIX), 0);
            return;
        }

        let x_bits = read_u32(matrix.wrapping_add(MATRIX_X));
        let y_bits = read_u32(matrix.wrapping_add(MATRIX_Y));
        let z_bits = read_u32(matrix.wrapping_add(MATRIX_Z));
        let first_angle = f32::from_bits(
            read_u32(matrix.wrapping_add(MATRIX_ANGLE_X)) ^ SIGN_BIT,
        ) as f64;
        let second_angle = f32::from_bits(read_u32(matrix.wrapping_add(MATRIX_ANGLE_Y))) as f64;
        let first_angle_bits = first_angle.to_bits();
        let second_angle_bits = second_angle.to_bits();
        let angle_bits = lf_checker_rt::callee_cdecl!(
            ATAN2,
            u64,
            first_angle_bits as u32,
            (first_angle_bits >> 32) as u32,
            second_angle_bits as u32,
            (second_angle_bits >> 32) as u32,
        );
        let angle = f64::from_bits(angle_bits) as f32;

        write_u32(this.wrapping_add(ENTITY_X), x_bits);
        write_u32(this.wrapping_add(ENTITY_Y), y_bits);
        write_u32(this.wrapping_add(ENTITY_Z), z_bits);
        write_u32(this.wrapping_add(ENTITY_ANGLE), angle.to_bits());
        write_u32(this.wrapping_add(ENTITY_MATRIX), 0);

        let manager = lf_checker_rt::relocated(MATRIX_MANAGER_VA);
        write_u32(matrix.wrapping_add(MATRIX_ACTIVE), 0);
        let _ = lf_checker_rt::callee_thiscall!(RELEASE_NODE, u32, manager, matrix);
    }
});
