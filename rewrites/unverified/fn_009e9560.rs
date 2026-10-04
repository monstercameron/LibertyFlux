// original: 0x009e9560 ped_active_task_matches
/// True when the object has the active flag (`+0x264 & 0x8000`),
/// its active task id is non-negative, and the resolved task id for
/// it equals that active id. (thiscall; low byte is the value.)
lf_checker_rt::export!(thiscall, rw_009e9560(this_ptr: u32) -> u32 {
    unsafe {
        const ACTIVE_OFF: u32 = 0x264;
        const ACTIVE_BIT: u32 = 0x8000;
        let flags = (this_ptr.wrapping_add(ACTIVE_OFF) as *const u32).read_unaligned();
        if flags & ACTIVE_BIT == 0 {
            return 0;
        }
        let active: u32 = lf_checker_rt::callee_thiscall!(1, u32, this_ptr);
        if (active as i32) < 0 {
            return 0;
        }
        let resolved: u32 = lf_checker_rt::callee_cdecl!(2, u32, this_ptr, 0);
        if resolved == active { 1 } else { 0 }
    }
});
