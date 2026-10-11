// original: 0x00694D10 rage::crCreatureComponentExtraDofs::vf4

/// Call the component helper with a stack-local copy of its upper field.
///
/// The component is in ECX and two values arrive on the stack. The helper
/// receives a pointer to a one-word temporary containing the component's
/// `+0x10` field in ECX, followed by both values on its stack. Its result is
/// returned; the method removes its two incoming stack arguments.
lf_checker_rt::export!(thiscall, rw_00694D10(component: u32, first_value: u32, second_value: u32) -> u32 {
    unsafe {
        const HELPER_FIELD_OFFSET: u32 = 0x10;
        let mut helper_field = *((component.wrapping_add(HELPER_FIELD_OFFSET)) as *const u32);
        let helper_field_pointer = (&mut helper_field as *mut u32) as u32;
        lf_checker_rt::callee_thiscall!(1, u32, helper_field_pointer, first_value, second_value)
    }
});
