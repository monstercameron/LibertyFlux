// original: 0x00cb5b90 task_spawner
/// Spawns the mode-selected task; 0 when the spawner is missing.
export!(thiscall, rw_cb5b90(this_ptr: u32) -> u32 {
    unsafe {
        let mode = ((this_ptr.wrapping_add(0x50)) as *const u32).read();
        let reg = (lf_checker_rt::global::<u32>(0x0167E2A0) as *const u32).read();
        if mode != 0 {
            let r: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
            let h: u32 = lf_checker_rt::callee_thiscall!(2, u32, reg);
            if h == 0 {
                return 0;
            }
            let v = (r & 0x3FF).wrapping_add(0x3E8);
            lf_checker_rt::callee_thiscall!(3, u32, h, v, 0, 0, 0x41000000)
        } else {
            let h: u32 = lf_checker_rt::callee_thiscall!(2, u32, reg);
            if h == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(
                4, u32, h, this_ptr.wrapping_add(0x20), this_ptr.wrapping_add(0x30))
        }
    }
});
