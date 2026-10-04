// original: 0x00661af0 rage::snEstablishSessionTask::snEstablishSessionTask
/// Establish-session task constructor: base init plus sub-object defaults.
///
/// thiscall/0, returns `this`. Runs the shared base initializer, installs the
/// class vtable, clears the state words and initializes the four embedded
/// sub-objects through their own constructors.
export!(thiscall, rw_00661af0(this_ptr: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this_ptr);
        ((this_ptr + 0x60) as *mut u32).write(0);
        ((this_ptr + 0x64) as *mut u32).write(0);
        ((this_ptr + 0x68) as *mut u32).write(0);
        let flags = (this_ptr + 0x6c) as *mut u8;
        flags.write(flags.read() | 1);
        ((this_ptr + 0x6d) as *mut u8).write(0);
        ((this_ptr) as *mut u32).write(relocated(0x00fe3380));
        ((this_ptr + 0x90) as *mut u32).write(u32::MAX);
        ((this_ptr + 0x94) as *mut u32).write(0);
        ((this_ptr + 0x98) as *mut u32).write(0);
        ((this_ptr + 0x9c) as *mut u32).write(u32::MAX);
        ((this_ptr + 0xa8) as *mut u32).write(0);
        ((this_ptr + 0xac) as *mut u32).write(0);
        callee_thiscall!(2, u32, this_ptr.wrapping_add(0xb0));
        callee_thiscall!(3, u32, this_ptr.wrapping_add(0xa0));
        callee_thiscall!(4, u32, this_ptr.wrapping_add(0xe8));
        ((this_ptr + 0x758) as *mut u32).write(0);
        ((this_ptr + 0x75c) as *mut u32).write(u32::MAX);
        ((this_ptr + 0x760) as *mut u32).write(0);
        ((this_ptr + 0x764) as *mut u32).write(u32::MAX);
        callee_thiscall!(5, u32, this_ptr.wrapping_add(0x768));
        this_ptr
    }
});
