// original: 0x008DAA30 CAddTxdReferenceDC::CAddTxdReferenceDC

/// Build a render command by writing the base token and specialized vtable, then calling the
/// intercepted base-reference routine with its static registry object and one stack argument.
/// The constructor returns the command pointer in EAX.
lf_checker_rt::export!(thiscall, rw_008daa30(this: u32, argument: u32) -> u32 {
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E81BC0;
    const SERIAL_COUNTER_VA: u32 = 0x010327A0;
    const REGISTRY_VA: u32 = 0x011D4EC8;
    const TOKEN_OFFSET: usize = 4;
    const TOKEN_MASK: u32 = 0x0000_3FFF;
    const CALLEE_ID: u32 = 51;
    unsafe {
        let object = this as *mut u8;
        let counter = lf_checker_rt::global::<u32>(SERIAL_COUNTER_VA);
        let previous_token = core::ptr::read_unaligned(object.wrapping_add(TOKEN_OFFSET).cast::<u32>());
        core::ptr::write_unaligned(object.cast::<u32>(), lf_checker_rt::relocated(BASE_VTABLE_VA));
        let serial = core::ptr::read_volatile(counter.cast_const());
        let token = previous_token ^ ((previous_token ^ serial) & TOKEN_MASK);
        core::ptr::write_unaligned(object.wrapping_add(TOKEN_OFFSET).cast::<u32>(), token);
        let serial_after_token = core::ptr::read_volatile(counter.cast_const());
        core::ptr::write_volatile(counter, serial_after_token.wrapping_add(1));
        core::ptr::write_unaligned(object.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));
        let _ = lf_checker_rt::callee_thiscall!(CALLEE_ID, u32, lf_checker_rt::relocated(REGISTRY_VA), argument);
    }
    this
});
