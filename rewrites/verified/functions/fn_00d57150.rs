// original: 0x00d57150 CCamCinematicCamMan::initialize_state

/// Initializes the cinematic camera manager fields, records two signed comparisons of the random helper result against 0x3fff, copies a global word, and returns the exact EAX value with its low byte set to one.
///
/// `this` is a 32-bit pointer to the CCamCinematicCamMan object; the object uses the shared camera layout.
/// Original calling convention: thiscall with 0 explicit stack argument(s).
lf_checker_rt::export!(thiscall, rw_00d57150(this: u32) -> u32 {
        #[inline(always)]
        unsafe fn write32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn read_global(file_va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() }
        }
        const CALLEE_PARAMETER_INTEGRATION: u32 = 1;
        const CALLEE_RANDOM: u32 = 2;
        const PARAM_BLOCK_OFFSET: u32 = 0x140;
        const FLAG_FIRST_OFFSET: u32 = 0x208;
        const FLAG_SECOND_OFFSET: u32 = 0x20a;
        const RESERVED_BYTE_OFFSET: u32 = 0x209;
        const FIELD_1F0_OFFSET: u32 = 0x1f0;
        const FIELD_1E8_OFFSET: u32 = 0x1e8;
        const FIELD_1E4_OFFSET: u32 = 0x1e4;
        const FIELD_1E0_OFFSET: u32 = 0x1e0;
        const FIELD_1F8_OFFSET: u32 = 0x1f8;
        const FIELD_GLOBAL_WORD_OFFSET: u32 = 0x1f4;
        const FIELD_20B_OFFSET: u32 = 0x20b;
        const FIELD_1FC_OFFSET: u32 = 0x1fc;
        const FIELD_200_OFFSET: u32 = 0x200;
        const FIELD_204_OFFSET: u32 = 0x204;
        const SIGNED_BOUND: i32 = 0x3fff;
        const ZERO_WORD: u32 = 0;
        const ZERO_BYTE: u8 = 0;
        const FIELD_20B_DEFAULT: u16 = 0x0100;
        const EAX_HIGH_BITS_MASK: u32 = 0xffff_ff00;
        const TRUE_IN_AL: u32 = 1;
        const GLOBAL_WORD_FILE_VA: u32 = 0x011735c4;
        let _ = lf_checker_rt::callee_thiscall!(CALLEE_PARAMETER_INTEGRATION, u32, this.wrapping_add(PARAM_BLOCK_OFFSET));
        unsafe {
            write32(this.wrapping_add(FIELD_1F0_OFFSET), u32::MAX);
            write32(this.wrapping_add(FIELD_1E8_OFFSET), ZERO_WORD);
            write32(this.wrapping_add(FIELD_1E4_OFFSET), ZERO_WORD);
            write32(this.wrapping_add(FIELD_1E0_OFFSET), ZERO_WORD);
            write32(this.wrapping_add(FIELD_1F8_OFFSET), ZERO_WORD);
        }
        let first_random = lf_checker_rt::callee_cdecl!(CALLEE_RANDOM, u32,);
        let first_is_below = i32::from_ne_bytes(first_random.to_ne_bytes()) < SIGNED_BOUND;
        unsafe {
            (this.wrapping_add(FLAG_FIRST_OFFSET) as *mut u8).write(u8::from(first_is_below));
            (this.wrapping_add(RESERVED_BYTE_OFFSET) as *mut u8).write(ZERO_BYTE);
        }
        let second_random = lf_checker_rt::callee_cdecl!(CALLEE_RANDOM, u32,);
        let second_is_below = i32::from_ne_bytes(second_random.to_ne_bytes()) < SIGNED_BOUND;
        unsafe { (this.wrapping_add(FLAG_SECOND_OFFSET) as *mut u8).write(u8::from(second_is_below)); }
        let global_word = unsafe { read_global(GLOBAL_WORD_FILE_VA) };
        unsafe {
            write32(this.wrapping_add(FIELD_GLOBAL_WORD_OFFSET), global_word);
            (this.wrapping_add(FIELD_20B_OFFSET) as *mut u16).write_unaligned(FIELD_20B_DEFAULT);
            write32(this.wrapping_add(FIELD_1FC_OFFSET), u32::MAX);
            write32(this.wrapping_add(FIELD_200_OFFSET), u32::MAX);
            write32(this.wrapping_add(FIELD_204_OFFSET), u32::MAX);
        }
        (global_word & EAX_HIGH_BITS_MASK) | TRUE_IN_AL
});
