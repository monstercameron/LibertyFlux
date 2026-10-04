// original: 0x00661530 rage::snDropGamersTask::snDropGamersTask
/// Drop-gamers task constructor: base init, vtable install, array defaults.
///
/// thiscall/0, returns `this`. Runs the shared base initializer, clears the
/// session links, installs the class vtable, fills the trailing gamer-slot
/// words with -1 and zeroes the following record block.
export!(thiscall, rw_00661530(this_ptr: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this_ptr);
        ((this_ptr + 0x60) as *mut u32).write(0);
        ((this_ptr + 0x64) as *mut u32).write(0);
        ((this_ptr + 0x68) as *mut u32).write(0);
        let flags = (this_ptr + 0x6c) as *mut u8;
        flags.write(flags.read() | 1);
        ((this_ptr + 0x6d) as *mut u8).write(0);
        ((this_ptr) as *mut u32).write(relocated(0x00fe3428));
        ((this_ptr + 0x90) as *mut u32).write(u32::MAX);
        ((this_ptr + 0x94) as *mut u32).write(u32::MAX);
        for i in 0..64u32 {
            ((this_ptr + 0x98 + i * 4) as *mut u32).write(u32::MAX);
        }
        for k in 0..32u32 {
            let row = (this_ptr + 0x198 + k * 16) as *mut u8;
            for j in 0..16u32 {
                row.add(j as usize).write(0);
            }
        }
        ((this_ptr + 0x398) as *mut u32).write(0);
        this_ptr
    }
});
