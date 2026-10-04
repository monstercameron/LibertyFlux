// original: 0x00a87f00 offset_thunk_plus4
/// Adjust `this` by 4 and tail-call the shared implementation.
///
/// Adds 4 to the object pointer and tail-calls the zeroing helper
/// (intercepted), returning its answer.
export!(thiscall, rw_00a87f00(this_obj: u32) -> u32 {
    unsafe { callee_thiscall!(1, u32, this_obj.wrapping_add(4)) }
});
