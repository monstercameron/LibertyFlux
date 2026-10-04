// original: 0x00661280 rage::snLeaveGamersFromRlineTask::snLeaveGamersFromRlineTask
/// Construct a leave-gamers task: base, vtable, cleared member table.
///
/// Runs the base constructor, clears the session/state words, installs
/// this class's vtable, and clears the member table at `+0xa0`..`+0x2a0`
/// (0x200 bytes) plus its count word. Returns `this`.
export!(thiscall, rw_00661280(this: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this);
        ((this + 0x60) as *mut u32).write(0);
        ((this + 0x64) as *mut u32).write(0);
        ((this + 0x68) as *mut u32).write(0);
        ((this + 0x6c) as *mut u8).write(((this + 0x6c) as *const u8).read() | 1);
        ((this + 0x6d) as *mut u8).write(0);
        (this as *mut u32).write(relocated(0xfe3510));
        ((this + 0x90) as *mut u32).write(0xffff_ffff);
        ((this + 0x94) as *mut u32).write(0);
        ((this + 0x98) as *mut u32).write(0);
        core::ptr::write_bytes((this + 0xa0) as *mut u8, 0, 0x200);
        ((this + 0x2a0) as *mut u32).write(0);
        this
    }
});

