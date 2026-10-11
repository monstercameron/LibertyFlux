// original: 0x008602C0 physics_reset_child_responses

/// Clears the three response words in each of two child records unless that
/// child has the protected flag at `+0xe8`. It then calls the object's helper
/// with its child pointer in ECX and zero vectors in XMM1 and XMM2, and writes
/// fixed response values into this object. The helper preserves the live
/// parent pointer in EDX; its return value is unused.
lf_checker_rt::export!(thiscall, rw_008602c0(this: u32) -> u32 {
    unsafe {
        const CHILD_CONTAINER: u32 = 0x0140;
        const FIRST_CHILD_SLOT: u32 = 0x002c;
        const SECOND_CHILD_SLOT: u32 = 0x0024;
        const PROTECTED_FLAGS: u32 = 0x00e8;
        const PROTECTED_BIT: u8 = 0x04;
        const FIRST_RESPONSE: u32 = 0x0164;
        const CALL_OBJECT: u32 = 0x000c;
        const RESPONSE_A: u32 = 0x0154;
        const RESPONSE_B: u32 = 0x015c;
        const RESPONSE_C: u32 = 0x0164;
        const RESPONSE_D: u32 = 0x016c;
        const RESPONSE_A_BITS: u32 = 0x4140_0000;
        const RESPONSE_C_BITS: u32 = 0x4188_0000;

        #[inline(always)]
        unsafe fn read_word(base: u32, offset: u32) -> u32 {
            unsafe { ((base.wrapping_add(offset)) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_byte(base: u32, offset: u32) -> u8 {
            unsafe { ((base.wrapping_add(offset)) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn write_word(base: u32, offset: u32, value: u32) {
            unsafe { ((base.wrapping_add(offset)) as *mut u32).write_unaligned(value) }
        }

        let child_container = read_word(this, CHILD_CONTAINER);
        for child_slot in [FIRST_CHILD_SLOT, SECOND_CHILD_SLOT] {
            let child = read_word(child_container, child_slot);
            if read_byte(child, PROTECTED_FLAGS) & PROTECTED_BIT == 0 {
                write_word(child, FIRST_RESPONSE, 0);
                write_word(child, FIRST_RESPONSE + 4, 0);
                write_word(child, FIRST_RESPONSE + 8, 0);
            }
        }

        let helper_object = read_word(this, CALL_OBJECT);
        let _unused_helper_result =
            lf_checker_rt::callee_thiscall!(1, u32, helper_object, 0u32, 0u32);

        write_word(this, RESPONSE_A, RESPONSE_A_BITS);
        write_word(this, RESPONSE_B, RESPONSE_A_BITS);
        write_word(this, RESPONSE_C, RESPONSE_C_BITS);
        write_word(this, RESPONSE_D, RESPONSE_C_BITS);
        0
    }
});
