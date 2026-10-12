// original: 0x006397A0 rage::ptxRule::set_name

/// Compare a proposed rule name with the field at `this + 0x124`. If the
/// comparison helper's low byte is zero, pass the same field and name pointer
/// to the setter; otherwise return the comparison helper's full EAX value.
/// The setter path returns that helper's full EAX value. The object is in ECX,
/// the name pointer is one stack argument, and the callee removes that word.
lf_checker_rt::export!(thiscall, rw_006397A0(this: u32, name_pointer: u32) -> u32 {
    const COMPARE_NAME: u32 = 1;
    const SET_NAME: u32 = 2;
    const NAME_FIELD: u32 = 0x124;

    let name_field = this.wrapping_add(NAME_FIELD);
    let compare_result =
        lf_checker_rt::callee_thiscall!(COMPARE_NAME, u32, name_field, name_pointer);

    if (compare_result as u8) != 0 {
        compare_result
    } else {
        lf_checker_rt::callee_thiscall!(SET_NAME, u32, name_field, name_pointer)
    }
});
