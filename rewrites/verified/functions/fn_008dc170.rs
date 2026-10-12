// original: 0x008DC170 CSetShaderGPRAlloc

/// Build a render-state command, replacing the low fourteen token bits with
/// the global serial and copying two caller word(s) to +0x8, +0xC. The
/// serial advances with 32-bit wrapping. ECX supplies the object; the function
/// returns that pointer in EAX and makes no outgoing calls.
lf_checker_rt::export!(thiscall, rw_008dc170(this: u32, value_0: u32, value_1: u32) -> u32 {
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E81A54;
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
        core::ptr::write_unaligned(command.wrapping_add(8usize).cast::<u32>(), value_0);
        core::ptr::write_unaligned(command.wrapping_add(12usize).cast::<u32>(), value_1);
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));
    }
    this
});
