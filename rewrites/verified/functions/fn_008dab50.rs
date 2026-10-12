// original: 0x008DAB50 CCustomShaderEffectDC::CCustomShaderEffectDC

/// Create a custom-shader-effect command. It stores the base token and vtable, clears the output word, reads an unsigned 16-bit count from the input at +4, and passes the input pointer, count, and command output slot to the intercepted helper. The helper's scripted out-parameter is then copied to +8; the command pointer is returned.
lf_checker_rt::export!(thiscall, rw_008dab50(this: u32, source: u32) -> u32 {
    const COUNTER_VA: u32 = 0x010327A0;
    const BASE_VTABLE_VA: u32 = 0x00E7E048;
    const COMMAND_VTABLE_VA: u32 = 0x00E8193C;
    const TOKEN_MASK: u32 = 0x0000_3FFF;
    const TOKEN_OFFSET: usize = 4;
    const CALLEE_ID: u32 = 64;
    unsafe {
        let object = this as *mut u8;
        let counter = lf_checker_rt::global::<u32>(COUNTER_VA);
        let previous_token = core::ptr::read_volatile(object.wrapping_add(TOKEN_OFFSET).cast::<u32>());
        core::ptr::write_volatile(object.cast::<u32>(), lf_checker_rt::relocated(BASE_VTABLE_VA));
        let serial = core::ptr::read_volatile(counter.cast_const());
        let token = previous_token ^ ((previous_token ^ serial) & TOKEN_MASK);
        core::ptr::write_volatile(object.wrapping_add(TOKEN_OFFSET).cast::<u32>(), token);
        let serial_after_token = core::ptr::read_volatile(counter.cast_const());
        core::ptr::write_volatile(counter, serial_after_token.wrapping_add(1));
        core::ptr::write_volatile(object.cast::<u32>(), lf_checker_rt::relocated(COMMAND_VTABLE_VA));

        let output_slot = object.wrapping_add(0x10);
        core::ptr::write_volatile(output_slot.cast::<u32>(), 0);
        let source_pointer = source as *const u8;
        let element_count = core::ptr::read_volatile(source_pointer.wrapping_add(4).cast::<u16>()) as u32;
        core::ptr::write_volatile(object.wrapping_add(0x0c).cast::<u32>(), element_count);
        let _ = lf_checker_rt::callee_cdecl!(CALLEE_ID, u32, source, element_count, output_slot as u32);
        let out_value = core::ptr::read_volatile(output_slot.cast::<u32>());
        core::ptr::write_volatile(object.wrapping_add(8).cast::<u32>(), out_value);
    }
    this
});
