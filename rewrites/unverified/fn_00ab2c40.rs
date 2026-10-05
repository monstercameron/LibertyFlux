// original: 0x00ab2c40 stream_remove_refs (proposed)

/// Detach one reference from each of the two owner lists.
///
/// Runs the list-remove callee on the sub-object at `+0x143044` and then
/// the second-remove callee on the sub-object at `+0xD010`, both with the
/// same argument. No return value.
///
/// Callees: 1 = first list remove (thiscall, one word),
/// 2 = second list remove (thiscall, one word).
///
/// Original: 0x00ab2c40 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ab2c40(this: u32, arg: u32) -> u32 {
    unsafe {
        const REMOVE_A: u32 = 1;
        const REMOVE_B: u32 = 2;
        const LIST_A: u32 = 0x143044;
        const LIST_B: u32 = 0x00D010;
        lf_checker_rt::callee_thiscall!(REMOVE_A, u32, this.wrapping_add(LIST_A), arg);
        lf_checker_rt::callee_thiscall!(REMOVE_B, u32, this.wrapping_add(LIST_B), arg);
        0
    }
});
