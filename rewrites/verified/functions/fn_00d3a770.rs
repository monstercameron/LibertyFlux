// original: 0x00D3A770 CDummyTask_Stationary::vf11

/// Return false in AL when the mode-specific direct helper accepts, or when the target's virtual method at slot 1 returns 3; otherwise return true except for state 2 combined with task code 0x15f. The function removes three stack arguments.
lf_checker_rt::export!(thiscall, rw_00d3a770(this: u32, data: u32, state: u32, target: u32) -> u32 {
    unsafe {
        const DIRECT_CHECK: u32 = 2;
        const TARGET_METHOD: u32 = 1;
        let data_ready = (data.wrapping_add(0x3a0) as *const u8).read_unaligned() == 0;
        if data_ready && state == 2 {
            let linked = (this.wrapping_add(0x1c) as *const u32).read_unaligned();
            if linked != 0 {
                let flags = (linked.wrapping_add(0x28) as *const u32).read_unaligned() & 0x3c0;
                let pending = (linked.wrapping_add(0xf50) as *const u32).read_unaligned();
                if flags == 0x80 && pending == 0 {
                    let _ = lf_checker_rt::callee_cdecl!(DIRECT_CHECK, u32,
                        (this.wrapping_add(0x10) as *const u32).read_unaligned(), linked);
                    return 1;
                }
            }
        }
        if target != 0 {
            let target_state = lf_checker_rt::callee_thiscall!(TARGET_METHOD, u32, target);
            if target_state == 3 {
                return 0;
            }
        }
        if state == 2 {
            return 1;
        }
        let task_code = (this.wrapping_add(0x14) as *const u32).read_unaligned();
        (task_code != 0x15f) as u32
    }
});
