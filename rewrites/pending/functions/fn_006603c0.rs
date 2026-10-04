// original: 0x006603c0 rage::snAddRemoteGamerTask::snAddRemoteGamerTask
/// Construct an add-remote-gamer task: base, roster, embedded peer task.
///
/// Runs the base constructor, clears the session/state words, installs
/// this class's vtable, constructs the name member at `+0x98`, clears the
/// 0x200-byte roster at `+0x330` plus its tail words, constructs the
/// embedded connect-to-peer task at `+0x538`, and clears the queued flag
/// at `+0x5a4`. Returns `this`.
export!(thiscall, rw_006603c0(this: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this);
        ((this + 0x60) as *mut u32).write(0);
        ((this + 0x64) as *mut u32).write(0);
        ((this + 0x68) as *mut u32).write(0);
        ((this + 0x6c) as *mut u8).write(((this + 0x6c) as *const u8).read() | 1);
        ((this + 0x6d) as *mut u8).write(0);
        (this as *mut u32).write(relocated(0xfe3244));
        ((this + 0x90) as *mut u8).write(0);
        ((this + 0x94) as *mut u32).write(0xffff_ffff);
        callee_thiscall!(2, u32, this + 0x98);
        core::ptr::write_bytes((this + 0x330) as *mut u8, 0, 0x200);
        ((this + 0x530) as *mut u32).write(0);
        ((this + 0x534) as *mut u32).write(0);
        callee_thiscall!(3, u32, this + 0x538);
        ((this + 0x5a4) as *mut u8)
            .write(((this + 0x5a4) as *const u8).read() & 0xfe);
        this
    }
});

