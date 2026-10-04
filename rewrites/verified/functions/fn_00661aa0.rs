// original: 0x00661aa0 rage::snModifyPresenceFlagsTask::vf3
/// Presence-flags task tick: countdown, then report the latched outcome.
///
/// thiscall/1, returns void. Counts the tick timer down (landing exactly on
/// zero latches expired); reports success for the cancelled state and failure
/// for anything else, staying silent once finished.
export!(thiscall, rw_00661aa0(this_ptr: u32, dt: u32) -> u32 {
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
        if state == 3 {
            let vtable = (this_ptr as *const u32).read();
            let target = ((vtable + 0x1c) as *const u32).read() as usize;
            let done: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(target);
            done(this_ptr, 1, 0);
        } else if state != 1 {
            let vtable = (this_ptr as *const u32).read();
            let target = ((vtable + 0x1c) as *const u32).read() as usize;
            let done: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(target);
            done(this_ptr, 0, 0);
        }
        0
    }
});
