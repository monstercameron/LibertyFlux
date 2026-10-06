// original: 0x00900d80 input_notify_maybe_twice (proposed)
/// Notify the listener once, or twice when the object head is set.
///
/// `this` points to the object. When its first word is non-zero the notify
/// callee is called with `this`, and then it is tail-called with `this` in
/// both cases; the tail answer is the return value. Thiscall with no stack
/// arguments.
export!(thiscall, rw_00900d80(this: u32) -> u32 {
    unsafe {
        const CALL_ID: u32 = 1;
        const TAIL_ID: u32 = 2;
        if (this as *const u32).read_unaligned() != 0 {
            let _: u32 = callee_thiscall!(CALL_ID, u32, this);
        }
        callee_thiscall!(TAIL_ID, u32, this)
    }
});
