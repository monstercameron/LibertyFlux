// original: 0x008DACA0 CDrawEntityDC::CDrawEntityDC

/// Create an entity render command from a 16-bit model identifier, five byte fields, and a source structure. It copies the source's word and two float bit patterns before a final word, calls the intercepted source helper with a snapshot of both pointer arguments, registers the identifier with the intercepted manager, and returns the object pointer.
lf_checker_rt::export!(thiscall, rw_008daca0(this: u32, model_id: u32, first_byte: u32, second_byte: u32, third_byte: u32, source: u32, fourth_byte: u32, fifth_byte: u32) -> u32 {
    const SERIAL_COUNTER_VA: u32 = 0x010327A0;
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E81878;
    const MANAGER_VA: u32 = 0x011D4EC8;
    const SERIAL_MASK: u32 = 0x0000_3FFF;
    const SOURCE_HELPER: u32 = 83;
    const MANAGER_HELPER: u32 = 84;
    unsafe {
        let command = this as *mut u8;
        let counter = lf_checker_rt::global::<u32>(SERIAL_COUNTER_VA);
        let prior_token = core::ptr::read_unaligned(command.wrapping_add(4).cast::<u32>());
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(BASE_VTABLE_VA));
        let serial = core::ptr::read_volatile(counter.cast_const());
        let token = prior_token ^ ((prior_token ^ serial) & SERIAL_MASK);
        core::ptr::write_unaligned(command.wrapping_add(4).cast::<u32>(), token);
        let serial_after_token = core::ptr::read_volatile(counter.cast_const());
        core::ptr::write_volatile(counter, serial_after_token.wrapping_add(1));

        core::ptr::write_unaligned(command.wrapping_add(0x32), first_byte as u8);
        core::ptr::write_unaligned(command.wrapping_add(0x33), second_byte as u8);
        core::ptr::write_unaligned(command.wrapping_add(0x34), third_byte as u8);
        core::ptr::write_unaligned(command.wrapping_add(0x35), fourth_byte as u8);
        core::ptr::write_unaligned(command.wrapping_add(0x36), fifth_byte as u8);
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));
        core::ptr::write_unaligned(command.wrapping_add(0x30).cast::<u16>(), model_id as u16);

        let source = source as *const u8;
        let first_word = core::ptr::read_volatile(source.wrapping_add(0x30).cast::<u32>());
        let first_float = core::ptr::read_volatile(source.wrapping_add(0x34).cast::<u32>());
        let second_float = core::ptr::read_volatile(source.wrapping_add(0x38).cast::<u32>());
        core::ptr::write_volatile(command.wrapping_add(0x20).cast::<u32>(), first_word);
        core::ptr::write_volatile(command.wrapping_add(0x24).cast::<u32>(), first_float);
        core::ptr::write_volatile(command.wrapping_add(0x28).cast::<u32>(), second_float);
        let last_word = core::ptr::read_volatile(source.wrapping_add(0x3c).cast::<u32>());
        core::ptr::write_volatile(command.wrapping_add(0x2c).cast::<u32>(), last_word);

        let _ = lf_checker_rt::callee_thiscall!(SOURCE_HELPER, u32, source as u32, this.wrapping_add(0x10));
        let _ = lf_checker_rt::callee_thiscall!(MANAGER_HELPER, u32, lf_checker_rt::relocated(MANAGER_VA), model_id);
    }
    this
});
