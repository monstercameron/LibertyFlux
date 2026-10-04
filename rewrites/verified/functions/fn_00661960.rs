// original: 0x00661960 rage::snModifyPresenceFlagsTask::vf2
/// Presence-flags task start: latch once, publish flags while live.
///
/// thiscall/0, returns void. Latches the started flag; while the session is
/// joining or joined, refreshes the cached presence bit from callee 1 and
/// either reports completion (flags already match) or publishes them through
/// callee 2. Any other outcome reports failure through the completion virtual.
export!(thiscall, rw_00661960(this_ptr: u32) -> u32 {
    unsafe {
        if ((this_ptr + 0x0c) as *const u32).read() == 0 {
            ((this_ptr + 0x0c) as *mut u32).write(1);
        }
        let session = ((this_ptr + 0x60) as *const u32).read();
        let state = ((session + 0x50) as *const u32).read();
        if state == 2 || state == 3 {
            let live: u32 = callee_thiscall!(1, u32, session);
            let cached = ((this_ptr + 0x98) as *const u8).read() & 1;
            if cached != (live as u8) {
                let pflags = (this_ptr + 0x9c) as *mut u8;
                pflags.write(pflags.read() & 0xfe);
            }
            let flags = ((this_ptr + 0x98) as *const u32).read();
            if flags == ((session + 0x538) as *const u32).read() {
                let vtable = (this_ptr as *const u32).read();
                let target = ((vtable + 0x1c) as *const u32).read() as usize;
                let done: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(target);
                done(this_ptr, 1, 0);
            } else {
                callee_thiscall!(2, u32, session.wrapping_add(0x48), flags,
                    this_ptr.wrapping_add(0x90));
            }
        } else {
            let vtable = (this_ptr as *const u32).read();
            let target = ((vtable + 0x1c) as *const u32).read() as usize;
            let done: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(target);
            done(this_ptr, 0, 0);
        }
        0
    }
});
