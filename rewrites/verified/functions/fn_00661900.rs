// original: 0x00661900 rage::snModifyPresenceFlagsTask::snModifyPresenceFlagsTask
/// Presence-flags task constructor: base init plus cleared flag words.
///
/// thiscall/0, returns `this`. Runs the shared base initializer, installs the
/// class vtable and clears the state and presence-flag words.
export!(thiscall, rw_00661900(this_ptr: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this_ptr);
        ((this_ptr + 0x60) as *mut u32).write(0);
        ((this_ptr + 0x64) as *mut u32).write(0);
        ((this_ptr + 0x68) as *mut u32).write(0);
        let flags = (this_ptr + 0x6c) as *mut u8;
        flags.write(flags.read() | 1);
        ((this_ptr + 0x6d) as *mut u8).write(0);
        ((this_ptr) as *mut u32).write(relocated(0x00fe3350));
        ((this_ptr + 0x90) as *mut u32).write(0);
        ((this_ptr + 0x94) as *mut u32).write(0);
        let pflags = (this_ptr + 0x9c) as *mut u8;
        pflags.write(pflags.read() | 1);
        ((this_ptr + 0x98) as *mut u32).write(0);
        this_ptr
    }
});
