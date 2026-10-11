// original: 0x005B7720 stream_type_code_to_group_flag

/// Map a stream type code from the inclusive range 0x32..=0x48 to its
/// compact group code and write a one-byte auxiliary flag through `flag_out`.
///
/// The selector is passed in ECX and the output pointer in EDX. Values below
/// or above the supported range return `u32::MAX` and leave the output byte
/// untouched. Within range, an internal type map selects one of nine cases:
/// cases 0, 1, 2, 5, 6, and 7 return 0, 1, 2, 7, 3, and 4 respectively and
/// write zero; case 3 returns 1 and writes one; case 4 returns 0 and writes
/// one; case 8 returns `u32::MAX` without writing. The range test is unsigned.
///
/// The function reads no object fields, globals, or floating-point state and
/// makes no calls. Only `flag_out[0]` is written on cases 0 through 7.
#[allow(clippy::missing_safety_doc)]
lf_checker_rt::export!(fastcall, rw_005B7720(type_code: u32, flag_out: u32) -> u32 {
    const TYPE_CODE_BASE: u32 = 0x32;
    const TYPE_CODE_SPAN: u32 = 22;
    const TYPE_TO_CASE: [u8; 23] = [
        0, 1, 2, 3, 4, 8, 8, 0, 1, 2, 3, 4, 8, 5, 8, 8, 8, 8, 5, 6, 6, 7, 7,
    ];
    const RETURN_BY_CASE: [u32; 9] = [0, 1, 2, 1, 0, 7, 3, 4, u32::MAX];
    const FLAG_BY_CASE: [u8; 9] = [0, 0, 0, 1, 1, 0, 0, 0, 0];

    let offset = type_code.wrapping_sub(TYPE_CODE_BASE);
    if offset > TYPE_CODE_SPAN {
        return u32::MAX;
    }
    let case_index = TYPE_TO_CASE[offset as usize] as usize;
    if case_index < 8 {
        unsafe { (flag_out as *mut u8).write(FLAG_BY_CASE[case_index]); }
    }
    RETURN_BY_CASE[case_index]
});
