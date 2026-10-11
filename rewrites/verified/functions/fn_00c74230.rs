// original: 0x00c74230 find_ped_type_by_name

/// Return zero when the validation helper accepts `name`; otherwise compare
/// the NUL-terminated, case-sensitive byte string against each entry in the
/// 23-name table and return its zero-based index. Return 23 when no entry
/// matches. The validation result is tested through AL only.
lf_checker_rt::export!(cdecl, rw_00c74230(name: u32) -> u32 {
    const VALIDATE_NAME: u32 = 1;
    const NAME_POINTERS: u32 = 0x0104_b6f8;
    const NAME_COUNT: u32 = 23;

    let validation = lf_checker_rt::callee_cdecl!(VALIDATE_NAME, u32, name);
    if (validation as u8) != 0 {
        return 0;
    }

    unsafe fn equal_c_strings(left: u32, right: u32) -> bool {
        let mut offset = 0u32;
        loop {
            let left_byte = unsafe { (left.wrapping_add(offset) as *const u8).read_unaligned() };
            let right_byte = unsafe { (right.wrapping_add(offset) as *const u8).read_unaligned() };
            if left_byte != right_byte {
                return false;
            }
            if left_byte == 0 {
                return true;
            }
            offset = offset.wrapping_add(1);
        }
    }

    unsafe {
        let table = lf_checker_rt::relocated(NAME_POINTERS);
        for index in 0..NAME_COUNT {
            let candidate = ((table.wrapping_add(index.wrapping_mul(4))) as *const u32)
                .read_unaligned();
            if equal_c_strings(name, candidate) {
                return index;
            }
        }
    }
    NAME_COUNT
});
