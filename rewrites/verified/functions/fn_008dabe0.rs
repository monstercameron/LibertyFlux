// original: 0x008DABE0 CDrawCurvedWindowDC

/// Build a curved-window render command. The source pointer supplies two words for offsets +8 and +0xc; the next two stack words are copied bit-for-bit to +0x10 and +0x14, and the final word goes to +0x18. The token and vtable are updated, and the command pointer is returned.
lf_checker_rt::export!(thiscall, rw_008dabe0(this: u32, source: u32, gain_bits: u32, opacity_bits: u32, curve_id: u32) -> u32 {
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E81CBC;
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
        core::ptr::write_unaligned(command.wrapping_add(0x18).cast::<u32>(), u32::MAX);
        let source = source as *const u8;
        let light_red = core::ptr::read_volatile(source.cast::<u32>());
        core::ptr::write_volatile(command.wrapping_add(8).cast::<u32>(), light_red);
        let light_green = core::ptr::read_volatile(source.wrapping_add(4).cast::<u32>());
        core::ptr::write_volatile(command.wrapping_add(0x0c).cast::<u32>(), light_green);
        core::ptr::write_volatile(command.wrapping_add(0x10).cast::<u32>(), gain_bits);
        let curve_id = if false { curve_id.wrapping_add(1) } else { curve_id };
        core::ptr::write_volatile(command.wrapping_add(0x18).cast::<u32>(), curve_id);
        core::ptr::write_volatile(command.wrapping_add(0x14).cast::<u32>(), opacity_bits);
    }
    this
});
