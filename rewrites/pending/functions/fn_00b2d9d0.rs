// original: 0x00b2d9d0 garage_any_active_hit
// 0xB2D9D0 garage_any_active_hit (cdecl/1 -> al).
//
// Reports whether any active (nonzero-kind) record's range test accepts the
// given point. The range test itself is stubbed by the checker.
export!(cdecl, rw_00b2d9d0(arg: u32) -> u8 {
    unsafe {
        let test: extern "thiscall" fn(u32, u32) -> u8 =
            core::mem::transmute(callee_addr(1) as usize);
        let mut rec = relocated(0x1660308);
        let end = relocated(0x16613E8);
        while rec < end {
            if *((rec + 0x48) as *const u8) != 0 && test(rec, arg) != 0 {
                return 1;
            }
            rec += 0x6C;
        }
        0
    }
});
