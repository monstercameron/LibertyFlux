// original: 0x005FB500 fragInstNMGta::vf13

/// Forward the method's three word arguments and fourth argument's exact
/// single-precision bit pattern to virtual slot `+0x48`. If the instance's
/// resource at `+0x64` has a helper pointer at `+0x19c`, call that helper with
/// the first three words and return its result; otherwise return the virtual
/// call's result. The fourth argument is forwarded bit-for-bit and is not
/// converted or used in arithmetic. This thiscall method consumes four stack
/// words. The contract disables stack-memory comparison for the original's
/// temporary scratch word; all forwarded words are checked in the call log.
lf_checker_rt::export!(thiscall, rw_005fb500(
    this: u32,
    first_word: u32,
    second_word: u32,
    third_word: u32,
    float_bits: u32,
) -> u32 {
    const INITIAL_SLOT: u32 = 1;
    const RESOURCE_POINTER: u32 = 0x64;
    const RESOURCE_HELPER: u32 = 0x19c;
    const FOLLOWUP_HELPER: u32 = 2;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let initial_result = lf_checker_rt::callee_thiscall!(
            INITIAL_SLOT,
            u32,
            this,
            first_word,
            second_word,
            third_word,
            float_bits,
        );
        let resource = read_u32(this.wrapping_add(RESOURCE_POINTER));
        if resource == 0 {
            return initial_result;
        }
        let helper = read_u32(resource.wrapping_add(RESOURCE_HELPER));
        if helper == 0 {
            return initial_result;
        }
        lf_checker_rt::callee_thiscall!(
            FOLLOWUP_HELPER,
            u32,
            helper,
            first_word,
            second_word,
            third_word,
        )
    }
});
