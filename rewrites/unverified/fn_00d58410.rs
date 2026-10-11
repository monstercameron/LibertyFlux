// original: 0x00d58410 CCamCinematography::initialize_state

/// Initializes the cinematography parameter defaults at named byte offsets and returns true in AL.
///
/// `this` is a 32-bit pointer to the CCamCinematography object; the object uses the shared camera layout.
/// Original calling convention: thiscall with 0 explicit stack argument(s).
lf_checker_rt::export!(thiscall, rw_00d58410(this: u32) -> u32 {
        #[inline(always)]
        unsafe fn write32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        const ZERO_WORD: u32 = 0;
        const LIMIT_DEFAULT: u32 = u32::MAX;
        const DURATION_DEFAULT: u32 = 1000;
        const RATIO_DEFAULT_BITS: u32 = 0x3e99_999a;
        const TEN_DEFAULT_BITS: u32 = 0x4120_0000;
        const THREE_DEFAULT_BITS: u32 = 0x4040_0000;
        const TRUE_IN_AL: u32 = 1;
        const FIELD_ZERO_A: u32 = 0x148;
        const FIELD_ZERO_B: u32 = 0x14c;
        const FIELD_ZERO_C: u32 = 0x150;
        const FIELD_LIMIT: u32 = 0x154;
        const FIELD_DURATION: u32 = 0x158;
        const FIELD_RATIO_BITS: u32 = 0x15c;
        const FIELD_TEN_BITS: u32 = 0x160;
        const FIELD_THREE_BITS: u32 = 0x164;
        unsafe {
            write32(this.wrapping_add(FIELD_ZERO_A), ZERO_WORD);
            write32(this.wrapping_add(FIELD_ZERO_B), ZERO_WORD);
            write32(this.wrapping_add(FIELD_ZERO_C), ZERO_WORD);
            write32(this.wrapping_add(FIELD_LIMIT), LIMIT_DEFAULT);
            write32(this.wrapping_add(FIELD_DURATION), DURATION_DEFAULT);
            write32(this.wrapping_add(FIELD_RATIO_BITS), RATIO_DEFAULT_BITS);
            write32(this.wrapping_add(FIELD_TEN_BITS), TEN_DEFAULT_BITS);
            write32(this.wrapping_add(FIELD_THREE_BITS), THREE_DEFAULT_BITS);
        }
        TRUE_IN_AL
});
