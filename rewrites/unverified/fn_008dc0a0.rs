// original: 0x008DC0A0 CSetColorWrite

/// Write a render command's vtable, serial token, and one caller-provided word.
/// The serial token replaces the low fourteen bits of the previous token with
/// the current global serial, then the serial advances with 32-bit wrapping.
/// The function takes the command object in ECX, one word on the stack, and
/// returns the object pointer in EAX. It performs no outgoing calls.
lf_checker_rt::export!(thiscall, rw_008dc0a0(this: u32, stored_value: u32) -> u32 {
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E81A70;
    const SERIAL_COUNTER_VA: u32 = 0x010327A0;
    const SERIAL_MASK: u32 = 0x0000_3FFF;
    const TOKEN_OFFSET: u32 = 4;
    const VALUE_OFFSET: u32 = 8;

    unsafe {
        let command = this as *mut u8;
        let serial_counter = lf_checker_rt::global::<u32>(SERIAL_COUNTER_VA);
        let prior_token = core::ptr::read_unaligned(command.wrapping_add(TOKEN_OFFSET as usize).cast::<u32>());
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(BASE_VTABLE_VA));
        let serial_for_token = core::ptr::read_volatile(serial_counter.cast_const());
        let token_delta = (prior_token ^ serial_for_token) & SERIAL_MASK;
        let token = prior_token ^ token_delta;
        core::ptr::write_unaligned(command.wrapping_add(TOKEN_OFFSET as usize).cast::<u32>(), token);
        let serial_after_token = core::ptr::read_volatile(serial_counter.cast_const());
        core::ptr::write_volatile(serial_counter, serial_after_token.wrapping_add(1));
        core::ptr::write_unaligned(command.wrapping_add(VALUE_OFFSET as usize).cast::<u32>(), stored_value);
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));
    }
    this
});
