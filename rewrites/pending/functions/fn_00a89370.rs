// original: 0x00a89370 flag_set_and_notify
/// Set the ready flag and notify the helper.
///
/// Writes 1 to the byte at `this + 0xA04`, then calls the helper
/// (intercepted, thiscall/2) on the sub-object at `this + 0x130` with
/// arguments (0xFFFF, 0). Returns the helper's answer.
export!(thiscall, rw_00a89370(this_obj: u32) -> u32 {
    unsafe {
        *((this_obj.wrapping_add(0xa04)) as *mut u8) = 1;
        callee_thiscall!(1, u32, this_obj.wrapping_add(0x130), 0xffff, 0)
    }
});
