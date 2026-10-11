// original: 0x009d0910 timing_object_flag_is_one

/// Return one for a null object. Otherwise call the one-word helper; a nonzero low byte selects the word at offset 0x34, while zero selects offset 0x40. Return one only when the selected word equals one.
lf_checker_rt::export!(cdecl, rw_009d0910(unused_word: u32, object: u32) -> u32 {
    const HELPER_ID: u32 = 1;
    const TRUE_FIELD_OFFSET: u32 = 0x34;
    const FALSE_FIELD_OFFSET: u32 = 0x40;
    if object == 0 {
        return 1;
    }
    let helper_answer = unsafe { lf_checker_rt::callee_cdecl!(HELPER_ID, u32, object) } as u8;
    let field_offset = if helper_answer != 0 { TRUE_FIELD_OFFSET } else { FALSE_FIELD_OFFSET };
    let field_value = unsafe { ((object.wrapping_add(field_offset)) as *const u32).read_unaligned() };
    u32::from(field_value == 1)
});
