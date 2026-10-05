// original: 0x00941530 streaming_matrix_setup (proposed)

/// Build the streaming rotation matrix from an angle and apply two updates.
///
/// `this + 0x20` points at a 0x2c-byte matrix, or is null. When null, the
/// third argument's bits are stored to `this + 0x1c` and the function
/// returns. Otherwise the first argument (an angle) is fed to the cosine
/// and sine helpers and the matrix is stamped as a rotation: 1.0 at +0,
/// cosine at +0x14 and +0x28, sine at +0x18, sine xored with the global
/// sign mask at +0x24, zero elsewhere. Then the second and third arguments
/// are applied through two update calls with the matrix as `this`.
/// Returns the last update's answer, or leaves `eax` untouched on the
/// null path.
///
/// Original: 0x00941530 (thiscall, three stack arguments; callee pops 12).
lf_checker_rt::export!(thiscall, rw_00941530(this: u32, angle: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const MATRIX: u32 = 0x20;
        const FALLBACK: u32 = 0x1C;
        const ONE: u32 = 0x3F80_0000;
        const SIGN_MASK: u32 = 0x00FE8FA0;
        const COS_CALLEE: u32 = 1;
        const SIN_CALLEE: u32 = 2;
        const UPD_A: u32 = 3;
        const UPD_B: u32 = 4;
        let matrix = ((this + MATRIX) as *const u32).read_unaligned();
        if matrix == 0 {
            ((this + FALLBACK) as *mut u32).write_unaligned(a2);
            return 0;
        }
        let cosv: u32 = lf_checker_rt::callee_cdecl!(COS_CALLEE, u32, angle);
        let sinv: u32 = lf_checker_rt::callee_cdecl!(SIN_CALLEE, u32, angle);
        ((matrix) as *mut u32).write_unaligned(ONE);
        ((matrix + 4) as *mut u32).write_unaligned(0);
        ((matrix + 8) as *mut u32).write_unaligned(0);
        ((matrix + 0x18) as *mut u32).write_unaligned(sinv);
        ((matrix + 0x10) as *mut u32).write_unaligned(0);
        ((matrix + 0x14) as *mut u32).write_unaligned(cosv);
        let mask = lf_checker_rt::global::<u32>(SIGN_MASK).read();
        ((matrix + 0x24) as *mut u32).write_unaligned(sinv ^ mask);
        ((matrix + 0x20) as *mut u32).write_unaligned(0);
        ((matrix + 0x28) as *mut u32).write_unaligned(cosv);
        let _: u32 = lf_checker_rt::callee_thiscall!(UPD_A, u32, matrix, a1);
        lf_checker_rt::callee_thiscall!(UPD_B, u32, matrix, a2)
    }
});
