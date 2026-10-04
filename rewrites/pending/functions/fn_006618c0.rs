// original: 0x006618c0 rage::snDestroyTask::vf3
/// Destroy task tick: countdown, then report unless already finished.
///
/// thiscall/1, returns void. Counts the tick timer down (landing exactly on
/// zero latches expired); unless the finished state latched, reports through
/// the task's own completion virtual, flagging the cancelled state.
export!(thiscall, rw_006618c0(this_ptr: u32, dt: u32) -> u32 {
    unsafe {
        let timer = (this_ptr + 0x14) as *mut u32;
        let left = timer.read();
        if (left as i32) > 0 {
            let next = left.wrapping_sub(dt);
            timer.write(next);
            if next == 0 {
                timer.write(u32::MAX);
            }
        }
        let state = ((this_ptr + 0x90) as *const u32).read();
        if state != 1 {
            let vtable = (this_ptr as *const u32).read();
            let target = ((vtable + 0x1c) as *const u32).read() as usize;
            let done: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(target);
            done(this_ptr, (state == 3) as u32, 0);
        }
        0
    }
});
