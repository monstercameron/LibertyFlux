// original: 0x00a2a0f0 player_ped_release_field_e64

/// If the pointer at `this+0xe64` is non-null, call its release helper with
/// ECX set to that pointer and the address of the field as the sole stack
/// argument, then clear the field. EAX is the helper's result, or zero if empty.
lf_checker_rt::export!(thiscall, rw_a2a0f0(this: u32) -> u32 {
    const HANDLE_FIELD: u32 = 0x0e64;
    let field = (this + HANDLE_FIELD) as *mut u32;
    let object = unsafe { field.read_unaligned() };
    if object == 0 {
        return 0;
    }
    let result = unsafe { lf_checker_rt::callee_thiscall!(1, u32, object, field as u32) };
    unsafe { field.write_unaligned(0) };
    result
});
