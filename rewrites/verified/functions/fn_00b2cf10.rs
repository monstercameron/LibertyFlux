// original: 0x00b2cf10 garage_tables_rebuild
// 0xB2CF10 garage_tables_rebuild (cdecl/0).
//
// Rebuilds the live tables: the first `count` records get their tag copied
// forward and their timer/counter/link cleared, then the shared record reset
// runs on each; the global counters and flags are zeroed (one stamped -1);
// finally the shared cell clearer runs over the two rows of ten cells.
export!(cdecl, rw_00b2cf10() -> () {
    unsafe {
        let reset: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let clear: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let count = *global::<u32>(0x165FD28);
        let mut i = 0u32;
        while i < count {
            let rec = relocated(0x1660350) + i * 0x6C;
            *(rec as *mut u8) = *((rec + 2) as *const u8);
            *((rec + 8) as *mut u32) = 0;
            *((rec + 0xC) as *mut u32) = 0;
            *((rec + 0x10) as *mut u32) = 0;
            reset(rec + 0x14);
            i += 1;
        }
        *global::<u32>(0x165FD30) = 0;
        *global::<u32>(0x165FD2C) = 0;
        *global::<u8>(0x165FD44) = 0;
        *global::<u8>(0x165FD45) = 0;
        *global::<u8>(0x165FD46) = 0;
        *global::<u8>(0x165FD47) = 0;
        *global::<u32>(0x165FD4C) = 0;
        *global::<u32>(0x165FD50) = 0xFFFF_FFFF;
        let mut row = relocated(0x165FD60);
        let rows_end = relocated(0x165FDF0);
        while row < rows_end {
            let mut cell = row;
            let mut n = 10u32;
            while n != 0 {
                clear(cell);
                cell += 0x90;
                n -= 1;
            }
            row += 0x48;
        }
    }
});
