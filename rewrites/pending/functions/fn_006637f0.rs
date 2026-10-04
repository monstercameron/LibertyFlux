// original: 0x006637f0 rage::snMigrateSessionTask::vf2
/// Migrate-session task start: latch once, rearm while the session is live.
///
/// thiscall/0, returns void. Latches the started flag; while the session is
/// joining or joined, rearms the migration counters through callee 1.
/// Otherwise reports through the task's own completion virtual.
export!(thiscall, rw_006637f0(this_ptr: u32) -> u32 {
    unsafe {
        if ((this_ptr + 0x0c) as *const u32).read() == 0 {
            ((this_ptr + 0x0c) as *mut u32).write(1);
        }
        let session = ((this_ptr + 0x60) as *const u32).read();
        let state = ((session + 0x50) as *const u32).read();
        if state == 2 || state == 3 {
            callee_thiscall!(1, u32, this_ptr);
            ((this_ptr + 0x1694) as *mut u32).write(0);
            ((this_ptr + 0x908) as *mut u32).write(u32::MAX);
            ((this_ptr + 0x90) as *mut u32).write(0);
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
