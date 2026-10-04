// original: 0x00b2dd80 garage_flags_commit
// 0xB2DD80 garage_flags_commit (cdecl/0).
//
// Sweeps all forty records in groups of five: a record of kind 5 whose
// status word carries bit 0x40 gets its pending byte set to 1.
export!(cdecl, rw_00b2dd80() -> () {
    unsafe {
        let mut group = relocated(0x1660354);
        let end = relocated(0x1661434);
        while group < end {
            let mut slot = 0u32;
            while slot < 5 {
                let flag = group + slot * 0x6C;
                if *((flag - 4) as *const u8) == 5 && *(flag as *const u8) & 0x40 != 0 {
                    *((flag - 3) as *mut u8) = 1;
                }
                slot += 1;
            }
            group += 0x21C;
        }
    }
});
