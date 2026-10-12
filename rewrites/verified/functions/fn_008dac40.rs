// original: 0x008DAC40 CDrawDefLight

/// Build a definition-light render command from six pointers, reading one word from each. The words are stored in order at command offsets +0x18, +0x1c, +8, +0xc, +0x10 and +0x14. The token and vtable are updated, and the command pointer is returned.
lf_checker_rt::export!(thiscall, rw_008dac40(this: u32, source_0: u32, source_1: u32, source_2: u32, source_3: u32, source_4: u32, source_5: u32) -> u32 {
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E81958;
    const SERIAL_COUNTER_VA: u32 = 0x010327A0;
    const SERIAL_MASK: u32 = 0x0000_3FFF;
    const TOKEN_OFFSET: usize = 4;

    unsafe {
        let command = this as *mut u8;
        let serial_counter = lf_checker_rt::global::<u32>(SERIAL_COUNTER_VA);
        let prior_token = core::ptr::read_unaligned(command.wrapping_add(TOKEN_OFFSET).cast::<u32>());
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(BASE_VTABLE_VA));
        let serial_for_token = core::ptr::read_volatile(serial_counter.cast_const());
        let token = prior_token ^ ((prior_token ^ serial_for_token) & SERIAL_MASK);
        core::ptr::write_unaligned(command.wrapping_add(TOKEN_OFFSET).cast::<u32>(), token);
        let serial_after_token = core::ptr::read_volatile(serial_counter.cast_const());
        core::ptr::write_volatile(serial_counter, serial_after_token.wrapping_add(1));
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));
        let source_0 = source_0 as *const u32;
        let value_0 = core::ptr::read_unaligned(source_0);
        core::ptr::write_unaligned(command.wrapping_add(0x18).cast::<u32>(), value_0);
        let source_1 = source_1 as *const u32;
        let value_1 = core::ptr::read_unaligned(source_1);
        core::ptr::write_unaligned(command.wrapping_add(0x1c).cast::<u32>(), value_1);
        let source_2 = source_2 as *const u32;
        let value_2 = core::ptr::read_unaligned(source_2);
        core::ptr::write_unaligned(command.wrapping_add(0x08).cast::<u32>(), value_2);
        let source_3 = source_3 as *const u32;
        let value_3 = core::ptr::read_unaligned(source_3);
        core::ptr::write_unaligned(command.wrapping_add(0x0c).cast::<u32>(), value_3);
        let source_4 = source_4 as *const u32;
        let value_4 = core::ptr::read_unaligned(source_4);
        core::ptr::write_unaligned(command.wrapping_add(0x10).cast::<u32>(), value_4);
        let source_5 = source_5 as *const u32;
        let value_5 = core::ptr::read_unaligned(source_5);
        core::ptr::write_unaligned(command.wrapping_add(0x14).cast::<u32>(), value_5);

    }
    this
});
