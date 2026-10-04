// original: 0x00a6f8b0 CTaskComplexPlayerInCover::vf20

/// Accessor hook of the in-cover player task: notifies the watcher of the
/// incoming argument, then returns this task's linked child.
///
/// The argument is forwarded to the notify routine (callee 1, thiscall with
/// `this` in ecx and the word on the stack; the callee pops it, which is
/// what keeps the original's push-heavy sequence balanced). Returns the
/// child pointer at `this + CHILD`.
///
/// Original: thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a6f8b0(this: u32, arg: u32) -> u32 {
    unsafe {
        const CHILD: u32 = 8;
        const NOTIFY: u32 = 1;

        lf_checker_rt::callee_thiscall!(NOTIFY, u32, this, arg);
        ((this as *const u32).wrapping_byte_offset(CHILD as isize)).read_unaligned()
    }
});
