// original: 0x005C43E0 ped_task_enabled_positive_count_predicate

/// Returns one when the field at `this + 0x30` is nonzero and the signed
/// 32-bit count at `this + 0x38` is positive. The object pointer arrives in
/// ECX, and the result is AL. The `jle` branch makes the count comparison
/// signed; negative values and zero return false.
lf_checker_rt::export!(thiscall, rw_005c43e0(this: u32) -> u32 {
    unsafe {
        const ENABLED_FIELD: u32 = 0x30;
        const SIGNED_COUNT: u32 = 0x38;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let enabled = read_word(this + ENABLED_FIELD) != 0;
        let count_is_positive = (read_word(this + SIGNED_COUNT) as i32) > 0;
        u32::from(enabled && count_is_positive)
    }
});
