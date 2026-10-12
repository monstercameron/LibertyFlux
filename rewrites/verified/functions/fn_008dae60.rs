// original: 0x008DAE60 CDrawFragTypeDC::CDrawFragTypeDC

/// Create a fragment-type render command from a 16-bit model identifier, three byte fields, two extra words, and an aliased source structure. The manager registration is intercepted; twelve source words are copied sequentially and the object pointer is returned.
lf_checker_rt::export!(thiscall, rw_008dae60(this: u32, model_id: u32, first_byte: u32, second_byte: u32, third_byte: u32, source: u32, extra_word: u32, trailing_word: u32) -> u32 {
    const SERIAL_COUNTER_VA: u32 = 0x010327A0;
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E81920;
    const MANAGER_VA: u32 = 0x011D4EC8;
    const SERIAL_MASK: u32 = 0x0000_3FFF;
    const MANAGER_CALLEE: u32 = 82;
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
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));
        let _ = lf_checker_rt::callee_thiscall!(MANAGER_CALLEE, u32, lf_checker_rt::relocated(MANAGER_VA), model_id);

        core::ptr::write_unaligned(command.wrapping_add(0x58).cast::<u16>(), model_id as u16);

        let source = source as *const u8;
        for (source_offset, target_offset) in [(0x0usize, 0x10usize), (4, 0x14), (8, 0x18), (0x10, 0x20), (0x14, 0x24), (0x18, 0x28), (0x20, 0x30), (0x24, 0x34), (0x28, 0x38), (0x30, 0x40), (0x34, 0x44), (0x38, 0x48)] {
            let word = core::ptr::read_volatile(source.wrapping_add(source_offset).cast::<u32>());
            core::ptr::write_volatile(command.wrapping_add(target_offset).cast::<u32>(), word);
        }
        core::ptr::write_unaligned(command.wrapping_add(0x50).cast::<u32>(), extra_word);
        core::ptr::write_unaligned(command.wrapping_add(0x5a), first_byte as u8);
        core::ptr::write_unaligned(command.wrapping_add(0x5b), second_byte as u8);
        core::ptr::write_unaligned(command.wrapping_add(0x5c), third_byte as u8);
        core::ptr::write_unaligned(command.wrapping_add(0x54).cast::<u32>(), trailing_word);
    }
    this
});
