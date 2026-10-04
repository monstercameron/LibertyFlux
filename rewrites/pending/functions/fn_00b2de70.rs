// original: 0x00b2de70 garage_substates_commit
// 0xB2DE70 garage_substates_commit (cdecl/0).
//
// Steps the sub-state machine of every active record, then zeroes the two
// pending counters.
export!(cdecl, rw_00b2de70() -> () {
    unsafe {
        let step: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let mut rec = relocated(0x1660308);
        let end = relocated(0x16613E8);
        while rec < end {
            if *((rec + 0x48) as *const u8) != 0 {
                step(rec);
            }
            rec += 0x6C;
        }
        *global::<u32>(0x165FD30) = 0;
        *global::<u32>(0x165FD2C) = 0;
    }
});
