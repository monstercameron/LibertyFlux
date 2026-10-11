// original: 0x00BDF0A0 CTaskComplexUseMobilePhone::CTaskComplexUseMobilePhone

/// Initialize the mobile-phone task's base subobject and task-specific
/// state. The first stack argument is copied to the word at byte offset
/// `0x14`; the remaining words at `0x18`, `0x1c`, `0x20` and `0x28` are
/// cleared, and the signed sentinel `-1` is stored at `0x24`. The object
/// pointer is returned. The base constructor call is forwarded through
/// the checker runtime.
lf_checker_rt::export!(thiscall, rw_00bdf0a0(this: u32, control: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EB9294;
        const CONTROL: usize = 0x14;
        const STATE_A: usize = 0x18;
        const STATE_B: usize = 0x1C;
        const STATE_WORD: usize = 0x20;
        const HANDLE: usize = 0x24;
        const FLAGS: usize = 0x28;

        let task = this as *mut u8;
        let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this);
        task.cast::<u32>().write(lf_checker_rt::relocated(VTABLE));
        task.add(CONTROL).cast::<u32>().write(control);
        task.add(STATE_A).cast::<u32>().write(0);
        task.add(STATE_B).cast::<u32>().write(0);
        task.add(STATE_WORD).cast::<u16>().write(0);
        task.add(HANDLE).cast::<u32>().write(u32::MAX);
        task.add(FLAGS).cast::<u32>().write(0);
        this
    }
});
