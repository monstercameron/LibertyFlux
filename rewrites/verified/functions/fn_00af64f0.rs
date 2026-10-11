// original: 0x00AF64F0 read_object_value_or_minus_one

/// Return minus one when the object has no value pointer; otherwise ask the
/// helper to write the value associated with that pointer into a local word
/// initially containing the object pointer, then return the resulting word.
/// The helper's output pointer refers to the current stack frame, so the
/// rewrite observes the output through a call-time snapshot in its contract.
lf_checker_rt::export!(thiscall, rw_00AF64F0(object: u32) -> u32 {
    const VALUE_POINTER_OFFSET: u32 = 0x24;

    let value_pointer = unsafe {
        (object.wrapping_add(VALUE_POINTER_OFFSET) as *const u32).read_volatile()
    };
    if value_pointer == 0 {
        return u32::MAX;
    }

    let mut output_word = object;
    let output_pointer = (&mut output_word as *mut u32) as u32;
    let _ignored_helper_result =
        lf_checker_rt::callee_cdecl!(1, u32, value_pointer, output_pointer);
    output_word
});
