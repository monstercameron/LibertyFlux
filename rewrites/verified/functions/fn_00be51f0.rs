// original: 0x00BE51F0 task_forward_virtual_float_to_helper

/// Read a floating value from a virtual method at slot +8, pass its exact
/// single-precision bits to the helper, and return the helper's integer result.
/// The value is forwarded without arithmetic; the contract supplies signed
/// zero, finite values, infinities, and a quiet NaN and compares the call word.
lf_checker_rt::export!(thiscall, rw_00be51f0(this: u32) -> u32 {
    unsafe {
        let object = ((this + 0x14) as *const u32).read_unaligned();
        let first_vtable = (object as *const u32).read_unaligned();
        let first_method = ((first_vtable + 0x30) as *const u32).read_unaligned();
        let get_float_source: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(first_method as usize);
        let float_source = get_float_source(object);

        let float_vtable = (float_source as *const u32).read_unaligned();
        let float_method = ((float_vtable + 8) as *const u32).read_unaligned();
        let read_float: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(float_method as usize);
        let measured_value = read_float(float_source);
        lf_checker_rt::callee_cdecl!(
            3,
            u32,
            core::hint::black_box(measured_value).to_bits()
        )
    }
});
