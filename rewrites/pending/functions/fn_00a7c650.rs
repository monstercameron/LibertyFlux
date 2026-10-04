// original: 0x00a7c650 bu_task_broadcast_all
/// Collects the subtree into a scratch array, then runs the shared helper
/// with the given argument on every collected node, innermost first.
export!(thiscall, rw_00a7c650(obj: *mut u8, arg: u32) -> u32 {
    unsafe {
        let mut buf = [0u32; 60];
        let n = callee_stdcall!(1, u32, buf.as_mut_ptr() as u32);
        let mut last = n;
        let mut i = n.wrapping_sub(1);
        if (i as i32) >= 0 {
            loop {
                last = callee_thiscall!(2, u32, buf[i as usize], arg);
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }
        last
    }
});
