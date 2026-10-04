// original: 0x00b2ced0 garage_tables_clear
// 0xB2CED0 garage_tables_clear (cdecl/0).
//
// Zeroes the two table counters and the per-record active/type bytes.
export!(cdecl, rw_00b2ced0() -> () {
    unsafe {
        *global::<u32>(0x165FD28) = 0;
        *global::<u32>(0x165FD54) = 0;
        let mut slot = relocated(0x1660352);
        let end = relocated(0x1661432);
        while slot < end {
            *((slot - 2) as *mut u8) = 0;
            *(slot as *mut u8) = 0;
            slot += 0x6C;
        }
    }
});
