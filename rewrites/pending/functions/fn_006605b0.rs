// original: 0x006605b0 task_state_copy
/// Copy a task's state block into this object.
///
/// Delegates the header region to the shared record copy, copies the
/// count word at `0x290`, then copies the 0x200-byte table at `0x298`
/// verbatim. Returns `this`.
export!(thiscall, rw_006605b0(this: u32, src: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this, src);
        ((this + 0x290) as *mut u32).write(((src + 0x290) as *const u32).read());
        core::ptr::copy_nonoverlapping(
            (src + 0x298) as *const u8,
            (this + 0x298) as *mut u8,
            0x200,
        );
        this
    }
});

