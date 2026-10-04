// original: 0x0065ff00 rage::snConnectToPeerTask::snConnectToPeerTask
/// Construct a connect-to-peer task: base, vtable, members, gamer slots.
///
/// Runs the base constructor, clears the session/state words, installs
/// this class's vtable, constructs the string member at `+0x98` and the
/// endpoint member at `+0xd0`, then marks the four gamer slots at
/// `+0x2c0`..`+0x2e8` empty (-1 ids, zero ports/flags). Returns `this`.
export!(thiscall, rw_0065ff00(this: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this);
        ((this + 0x60) as *mut u32).write(0);
        ((this + 0x64) as *mut u32).write(0);
        ((this + 0x68) as *mut u32).write(0);
        ((this + 0x6c) as *mut u8).write(((this + 0x6c) as *const u8).read() | 1);
        ((this + 0x6d) as *mut u8).write(0);
        (this as *mut u32).write(relocated(0xfe32c0));
        ((this + 0x90) as *mut u32).write(0xffff_ffff);
        callee_thiscall!(2, u32, this + 0x98);
        callee_thiscall!(3, u32, this + 0xd0);
        for slot in 0..4u32 {
            let base = this + 0x2c0 + slot * 8;
            (base as *mut u32).write(0xffff_ffff);
            ((base + 4) as *mut u16).write(0);
        }
        ((this + 0x2e0) as *mut u32).write(0xffff_ffff);
        ((this + 0x2e4) as *mut u32).write(0);
        ((this + 0x2e8) as *mut u32).write(0);
        this
    }
});

