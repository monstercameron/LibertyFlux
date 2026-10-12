// original: 0x008DAF10 CDrawMobilePhoneCameraDC::CDrawMobilePhoneCameraDC

/// Create a mobile-camera render command. The seven stack arguments provide a word, a helper input pointer, two source structures, a float bit pattern, and two byte values; the command copies them to the documented fields, updates its serial token, and returns the object pointer.
lf_checker_rt::export!(thiscall, rw_008daf10(this: u32, first_field: u32, helper_input: u32, source_words: u32, secondary_words: u32, scalar_bits: u32, first_byte: u32, second_byte: u32) -> u32 {
    const SERIAL_COUNTER_VA: u32 = 0x010327A0;
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E81D10;
    const SERIAL_MASK: u32 = 0x00003FFF;
    const TOKEN_OFFSET: usize = 4;
    const HELPER_A: u32 = 68;
    const HELPER_B: u32 = 69;
    unsafe {
        let command = this as *mut u8;
        let counter = lf_checker_rt::global::<u32>(SERIAL_COUNTER_VA);
        let prior_token = core::ptr::read_unaligned(command.wrapping_add(TOKEN_OFFSET).cast::<u32>());
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(BASE_VTABLE_VA));
        let serial = core::ptr::read_volatile(counter.cast_const());
        let token = prior_token ^ ((prior_token ^ serial) & SERIAL_MASK);
        core::ptr::write_unaligned(command.wrapping_add(TOKEN_OFFSET).cast::<u32>(), token);
        let serial_after_token = core::ptr::read_volatile(counter.cast_const());
        core::ptr::write_volatile(counter, serial_after_token.wrapping_add(1));
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));
        let _ = lf_checker_rt::callee_thiscall!(HELPER_A, u32, this.wrapping_add(0x10));
        core::ptr::write_unaligned(command.wrapping_add(8).cast::<u32>(), first_field);
        let _ = lf_checker_rt::callee_thiscall!(HELPER_B, u32, this.wrapping_add(0x10), helper_input);
        let first_source = source_words as *const u8;
        for offset in [0x0usize, 4, 8, 12] {
            let word = core::ptr::read_unaligned(first_source.wrapping_add(offset).cast::<u32>());
            core::ptr::write_unaligned(command.wrapping_add(0x400 + offset).cast::<u32>(), word);
        }
        let second_source = secondary_words as *const u8;
        for offset in [0x0usize, 4] {
            let word = core::ptr::read_unaligned(second_source.wrapping_add(offset).cast::<u32>());
            core::ptr::write_unaligned(command.wrapping_add(0x410 + offset).cast::<u32>(), word);
        }
        core::ptr::write_unaligned(command.wrapping_add(0x418).cast::<u32>(), scalar_bits);
        core::ptr::write_unaligned(command.wrapping_add(0x41c), first_byte as u8);
        core::ptr::write_unaligned(command.wrapping_add(0x41d), second_byte as u8);
    }
    this
});
