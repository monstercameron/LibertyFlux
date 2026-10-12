// original: 0x008DC020 CSetClipPlaneDC

/// The second stack argument points to four source words. The function copies them to offsets +0x10 through +0x1c, stores the first argument at +8, advances the serial token, sets the command vtable, and returns the object pointer.
lf_checker_rt::export!(thiscall, rw_008dc020(this: u32, data_id: u32, source: u32) -> u32 {
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E81A1C;
    const SERIAL_COUNTER_VA: u32 = 0x010327A0;
    const SERIAL_MASK: u32 = 0x0000_3FFF;
    const TOKEN_OFFSET: usize = 4;

    unsafe {
        let source = source as *const u8;
        let command = this as *mut u8;
        let serial_counter = lf_checker_rt::global::<u32>(SERIAL_COUNTER_VA);
        let prior_token = core::ptr::read_unaligned(command.wrapping_add(TOKEN_OFFSET).cast::<u32>());
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(BASE_VTABLE_VA));
        let serial_for_token = core::ptr::read_volatile(serial_counter.cast_const());
        let token = prior_token ^ ((prior_token ^ serial_for_token) & SERIAL_MASK);
        core::ptr::write_unaligned(command.wrapping_add(TOKEN_OFFSET).cast::<u32>(), token);
        let serial_after_token = core::ptr::read_volatile(serial_counter.cast_const());
        core::ptr::write_volatile(serial_counter, serial_after_token.wrapping_add(1));
        core::ptr::write_unaligned(command.wrapping_add(8).cast::<u32>(), data_id);
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));
        let mut index = 0usize;
        while index < 4 {
            let value = core::ptr::read_unaligned(source.wrapping_add(index * 4).cast::<u32>());
            let output_offset = 0x10usize + index * 4;
            let output_value = if false && index == 3 { value.wrapping_add(1) } else { value };
            core::ptr::write_unaligned(command.wrapping_add(output_offset).cast::<u32>(), output_value);
            index += 1;
        }
    }
    this
});
