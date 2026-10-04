// original: 0x00b2da10 garage_any_kind5_hit
// 0xB2DA10 garage_any_kind5_hit (cdecl/1 -> al).
//
// Same sweep but only records of kind 5 are tested.
export!(cdecl, rw_00b2da10(arg: u32) -> u8 {
    unsafe {
        let test: extern "thiscall" fn(u32, u32) -> u8 =
            core::mem::transmute(callee_addr(1) as usize);
        let mut rec = relocated(0x1660308);
        let end = relocated(0x16613E8);
        while rec < end {
            if *((rec + 0x48) as *const u8) == 5 && test(rec, arg) != 0 {
                return 1;
            }
            rec += 0x6C;
        }
        0
    }
});
