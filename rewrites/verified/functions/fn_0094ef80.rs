// original: 0x0094ef80 field_pair_compare_update_far
/// Compares two widely separated fields of the object and refreshes them.
///
/// Same shape as the +6/+0x86 variant: the pairwise check runs over the field
/// at +0x80 and the field at +0x100, and a zero low byte triggers the updater
/// with a full-mask flag and yields 1, otherwise 0.
export!(thiscall, rw_0094ef80(this_obj: u32) -> u32 {
    unsafe {
        const FIRST_OFF: u32 = 0x80;
        const SECOND_OFF: u32 = 0x100;
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
