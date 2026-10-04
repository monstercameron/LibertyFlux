// original: 0x009f1230 ped_inner_notify
/// Forward a fixed notification to the inner object at field `0x78`.
///
/// Loads the inner pointer and calls its handler with the constant
/// arguments (0, 0x10, 1), returning the handler's answer.
export!(thiscall, rw_009f1230(this_ptr: u32) -> u32 {
    unsafe {
        let inner = *((this_ptr + 0x78) as *const u32);
        callee_thiscall!(2, u32, inner, 0, 0x10, 1)
    }
});
