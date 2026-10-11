// original: 0x00694D90 rage::crCreatureComponentExtraDofs::vf8

/// Dispatch a component through the matching or fallback helper.
///
/// The component is in ECX and points to two objects at `+0x0c` and `+0x10`.
/// If the first object's `+8` field is nonzero and matches the second object's
/// `+8` field, the matching helper is called; otherwise the fallback helper is
/// called. The selected helper receives a stack-local copy of the first object
/// in ECX and the second object on its stack. Its EAX result is returned.
lf_checker_rt::export!(thiscall, rw_00694D90(component: u32) -> u32 {
    unsafe {
        const FIRST_OBJECT_OFFSET: u32 = 0x10;
        const SECOND_OBJECT_OFFSET: u32 = 0x0C;
        const KIND_OFFSET: u32 = 8;

        let first_object = *((component.wrapping_add(FIRST_OBJECT_OFFSET)) as *const u32);
        let second_object = *((component.wrapping_add(SECOND_OBJECT_OFFSET)) as *const u32);
        let first_kind = *((first_object.wrapping_add(KIND_OFFSET)) as *const u32);
        let mut helper_object = first_object;
        let helper_object_pointer = (&mut helper_object as *mut u32) as u32;

        if first_kind != 0
            && first_kind == *((second_object.wrapping_add(KIND_OFFSET)) as *const u32)
        {
            lf_checker_rt::callee_thiscall!(1, u32, helper_object_pointer, second_object, 0, 0)
        } else {
            lf_checker_rt::callee_thiscall!(2, u32, helper_object_pointer, second_object, 0, 0)
        }
    }
});
