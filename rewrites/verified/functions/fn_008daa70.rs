// original: 0x008DAA70 CClearRenderTargetDC::CClearRenderTargetDC

/// Build a clear-render-target command. Three stack arguments contribute their low bytes at +0x14, +0x15, and +0x16; the resource identifier, blend bits, and final mode word are stored at +8, +0xc, and +0x10. The float is copied bit-for-bit. The function advances its serial token and returns the command pointer.
lf_checker_rt::export!(thiscall, rw_008daa70(this: u32, flags_low: u32, resource_id: u32, flags_high: u32, blend_bits: u32, texture_flags: u32, blend_mode: u32) -> u32 {
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E81A00;
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
        let command = object;
        core::ptr::write_unaligned(command.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));
        core::ptr::write_unaligned(command.wrapping_add(8).cast::<u32>(), u32::MAX);
        core::ptr::write(command.wrapping_add(0x14), flags_low as u8);
        core::ptr::write_unaligned(command.wrapping_add(8).cast::<u32>(), resource_id);
        core::ptr::write(command.wrapping_add(0x15), flags_high as u8);
        core::ptr::write(command.wrapping_add(0x16), texture_flags as u8);
        core::ptr::write_unaligned(command.wrapping_add(0x10).cast::<u32>(), blend_mode);
        core::ptr::write_unaligned(command.wrapping_add(0x0c).cast::<u32>(), blend_bits);
    }
    this
});
