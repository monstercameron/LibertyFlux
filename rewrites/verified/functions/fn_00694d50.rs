// original: 0x00694D50 rage::crCreatureComponentExtraDofs::vf6

/// Forward an ExtraDofs component field and two values to its helper.
///
/// The component is in ECX and has a field pointer at `+0x0c`. The first
/// stack argument becomes the helper's ECX value; the field pointer, zero and
/// the second stack argument follow on its stack. The helper result is
/// returned, and this method removes both incoming stack arguments.
lf_checker_rt::export!(thiscall, rw_00694D50(component: u32, helper_this: u32, value: u32) -> u32 {
    unsafe {
        const FIELD_POINTER_OFFSET: u32 = 0x0C;
        let field_pointer = *((component.wrapping_add(FIELD_POINTER_OFFSET)) as *const u32);
        lf_checker_rt::callee_thiscall!(1, u32, helper_this, field_pointer, 0, value)
    }
});
