// original: 0x008DC350 CViewport_Callback_DC

/// The function copies two qword arguments from the stack into command offsets +8 and +0x10, stores one word at +0x18, advances the serial token, sets the vtable, and returns the object pointer. The qwords are transported as integer bit patterns.
lf_checker_rt::export!(thiscall, rw_008dc350(this: u32, scalar: u32, first_low: u32, first_high: u32, second_low: u32, second_high: u32) -> u32 {
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E81B18;
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
        let first_qword = first_low as u64 | ((first_high as u64) << 32);
        let second_qword = second_low as u64 | ((second_high as u64) << 32);
        core::ptr::write_unaligned(command.wrapping_add(8).cast::<u64>(), first_qword);
        core::ptr::write_unaligned(command.wrapping_add(0x18).cast::<u32>(), scalar);
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));
        core::ptr::write_unaligned(command.wrapping_add(0x10).cast::<u64>(), second_qword);
    }
    this
});
