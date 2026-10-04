// original: 0x00a87d90 subobject_reset_and_invalidate
/// Reset the sub-object at `this + 0x8DC` and mark it invalid.
///
/// Calls the reset helper (intercepted) with the sub-object address, then
/// writes -1 over its first dword. Returns the helper's answer.
export!(thiscall, rw_00a87d90(this_obj: u32) -> u32 {
    unsafe {
        let sub = this_obj.wrapping_add(0x8dc);
        let answer: u32 = callee_thiscall!(1, u32, sub);
        *(sub as *mut u32) = 0xffffffff;
        answer
    }
});
