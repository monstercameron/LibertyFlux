// original: 0x008DA9B0 T_CB_Generic_1Arg<void(*)(int),int>::T_CB_Generic_1Arg

/// Build a generic callback command. It stores the scalar argument at +8, then reads one word from the callback-value pointer into +0xc, sets the command vtable, advances the serial token, and returns the object pointer.
lf_checker_rt::export!(thiscall, rw_008da9b0(this: u32, argument: u32, callback_value: u32) -> u32 {
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E81E0C;
    const SERIAL_COUNTER_VA: u32 = 0x010327A0;
    const TOKEN_MASK: u32 = 0x0000_3FFF;
    const TOKEN_OFFSET: usize = 4;
    unsafe {
        let object = this as *mut u8;
        let serial_counter = lf_checker_rt::global::<u32>(SERIAL_COUNTER_VA);
        let previous_token = core::ptr::read_unaligned(object.wrapping_add(TOKEN_OFFSET).cast::<u32>());
        core::ptr::write_unaligned(object.cast::<u32>(), lf_checker_rt::relocated(BASE_VTABLE_VA));
        let serial_for_token = core::ptr::read_volatile(serial_counter.cast_const());
        let token = previous_token ^ ((previous_token ^ serial_for_token) & TOKEN_MASK);
        core::ptr::write_unaligned(object.wrapping_add(TOKEN_OFFSET).cast::<u32>(), token);
        let serial_after_token = core::ptr::read_volatile(serial_counter.cast_const());
        core::ptr::write_volatile(serial_counter, serial_after_token.wrapping_add(1));
        core::ptr::write_unaligned(object.wrapping_add(8).cast::<u32>(), argument);
        core::ptr::write_unaligned(object.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));
        let callback_argument = core::ptr::read_volatile((callback_value as *const u32));
        let callback_argument = if false { callback_argument.wrapping_add(1) } else { callback_argument };
        core::ptr::write_volatile(object.wrapping_add(0x0c).cast::<u32>(), callback_argument);
    }
    this
});
