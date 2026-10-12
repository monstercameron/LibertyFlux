// original: 0x008DABB0 CDataBlockDC::CDataBlockDC

/// Build a data-block command by storing one stack word at +8, updating the serial token and vtable, and returning the command pointer.
lf_checker_rt::export!(thiscall, rw_008dabb0(this: u32, value: u32) -> u32 {
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E8185C;
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
        let value = value;
        core::ptr::write_unaligned(object.wrapping_add(8).cast::<u32>(), value);
        core::ptr::write_unaligned(object.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));
    }
    this
});
