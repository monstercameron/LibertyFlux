// original: 0x0094ef50 field_pair_compare_update
/// Compares two fields of the object and refreshes them when they match.
///
/// Runs the pairwise check over the field at +6 and the field at +0x86. When
/// the check reports equal (low byte zero) the updater runs over the same two
/// fields with a full-mask flag and the result is 1; otherwise the result is 0.
export!(thiscall, rw_0094ef50(this_obj: u32) -> u32 {
    unsafe {
        const FIRST_OFF: u32 = 6;
        const SECOND_OFF: u32 = 0x86;
        const FULL_MASK: u32 = 0xFFFF_FFFF;
        let first = this_obj.wrapping_add(FIRST_OFF);
        let second = this_obj.wrapping_add(SECOND_OFF);
        let verdict = callee_cdecl!(1, u32, first, second);
        if verdict & 0xFF != 0 {
            return 0;
        }
        callee_cdecl!(2, u32, first, second, FULL_MASK);
        1
    }
});
