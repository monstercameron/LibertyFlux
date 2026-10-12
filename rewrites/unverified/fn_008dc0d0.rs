// original: 0x008DC0D0 CSetCurrentViewportDC::CSetCurrentViewportDC

/// Create a current-viewport render command and initialize its embedded viewport state. A non-null input is forwarded to the intercepted helper and leaves the byte at +0x400 clear; a null input skips that helper and sets the byte. The object pointer is returned.
lf_checker_rt::export!(thiscall, rw_008dc0d0(this: u32, input: u32) -> u32 {
    const SERIAL_COUNTER_VA: u32 = 0x010327A0;
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E819E4;
    const SERIAL_MASK: u32 = 0x0000_3FFF;
    const VIEWPORT_HELPER: u32 = 80;
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
        let _ = lf_checker_rt::callee_thiscall!(VIEWPORT_HELPER, u32, this.wrapping_add(0x10));
        let optional_input = input;
        core::ptr::write_unaligned(command.wrapping_add(0x400), if optional_input == 0 { 1 } else { 0 });
        if optional_input != 0 {
            let _ = lf_checker_rt::callee_thiscall!(81, u32, this.wrapping_add(0x10), optional_input);
        }
    }
    this
});
