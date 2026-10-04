// original: 0x009094A0 ui_slot_sweep (proposed) -- STAGE 1 (not verified).
//
// Stage 1 of the 1500-slot object-table sweep: kill-flag exit, the two
// pre-loop callees, the sweep loop and its per-slot exits, with an all-NULL
// table. A non-NULL slot faults loudly (contract keeps them unreachable).
// See the lane report for the full structure and the remaining stages.
lf_checker_rt::export!(cdecl, rw_009094A0_stage1() -> u32 {
    unsafe {
        const KILL: u32 = 0x0118_DC40;
        const MM: u32 = 0x0103_4494;
        const TAB: u32 = 0x0118_F6F8;
        const COUNT: i32 = 0x5DC;
        const PRE1: u32 = 1;
        const PRE2: u32 = 2;

        let kill = (lf_checker_rt::relocated(KILL) as *const u8).read();
        if kill != 0 {
            return 0;
        }
        let _pre = lf_checker_rt::callee_cdecl!(PRE1, u32,);
        let mut slot = 0u32;
        lf_checker_rt::callee_cdecl!(PRE2, u32, &mut slot as *mut u32 as u32);
        let m = (lf_checker_rt::relocated(MM) as *const i32).read();
        let tab = lf_checker_rt::relocated(TAB) as *const u32;
        let mut edi = 0i32;
        while edi < COUNT {
            if edi != m {
                let ecx = tab.wrapping_add(edi as usize).read();
                if ecx != 0 {
                    unreachable!("stage 1: table must be all NULL");
                }
            }
            edi += 1;
        }
        0
    }
});
