// original: 0x00661800 rage::snDestroyTask::vf2
/// Destroy task start: latch once, submit while the session is live.
///
/// thiscall/0, returns void. Latches the started flag, and while the session
/// state is joining or joined submits this task through callee 1; any other
/// outcome reports through the task's own completion virtual.
export!(thiscall, rw_00661800(this_ptr: u32) -> u32 {
    unsafe {
        if ((this_ptr + 0x0c) as *const u32).read() == 0 {
            ((this_ptr + 0x0c) as *mut u32).write(1);
        }
        let session = ((this_ptr + 0x60) as *const u32).read();
        let state = ((session + 0x50) as *const u32).read();
        if state == 2 || state == 3 {
            let ok: u32 =
                callee_thiscall!(1, u32, session.wrapping_add(0x48), this_ptr.wrapping_add(0x90));
            if (ok as u8) != 0 {
                return 0;
            }
        }
        let vtable = (this_ptr as *const u32).read();
        let target = ((vtable + 0x1c) as *const u32).read() as usize;
        let done: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(target);
        done(this_ptr, 0, 0);
        0
    }
});
