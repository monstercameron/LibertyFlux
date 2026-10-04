// original: 0x00662920 rage::snJoinSessionTask::vf2
/// Join-session task start: latch once, reset progress, report while live.
///
/// thiscall/0, returns void. Latches the started flag and resets the progress
/// word; reports through the task's own completion virtual while the session
/// is joining or joined, and otherwise parks the state at idle.
export!(thiscall, rw_00662920(this_ptr: u32) -> u32 {
    unsafe {
        if ((this_ptr + 0x0c) as *const u32).read() == 0 {
            ((this_ptr + 0x0c) as *mut u32).write(1);
        }
        ((this_ptr + 0x568) as *mut u32).write(0);
        let session = ((this_ptr + 0x60) as *const u32).read();
        let state = ((session + 0x50) as *const u32).read();
        if state == 2 || state == 3 {
            let vtable = (this_ptr as *const u32).read();
            let target = ((vtable + 0x1c) as *const u32).read() as usize;
            let done: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(target);
            done(this_ptr, 0, 0);
        } else {
            ((this_ptr + 0x90) as *mut u32).write(0);
        }
        0
    }
});
