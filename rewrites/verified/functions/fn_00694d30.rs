// original: 0x00694D30 rage::crCreatureComponentExtraDofs::vf5

/// Forward the component's upper field and two incoming values to a helper.
///
/// The component is in ECX and the values are two stack arguments. The helper
/// receives a pointer to those two values in ECX, plus the component field at
/// `+0x10` and the second value on its stack. Its EAX result is returned.
lf_checker_rt::export!(thiscall, rw_00694D30(component: u32, first_value: u32, second_value: u32) -> u32 {
    unsafe {
        const HELPER_FIELD_OFFSET: u32 = 0x10;
        let field = *((component.wrapping_add(HELPER_FIELD_OFFSET)) as *const u32);
        let values = [first_value, second_value];
        let values_pointer = values.as_ptr() as u32;
        lf_checker_rt::callee_thiscall!(1, u32, values_pointer, field, second_value)
    }
});
