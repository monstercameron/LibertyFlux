// original: 0x00B22D20 interior_pools_free
/// Release the portal index table and up to two record pools. Pool pointers
/// refer to allocation headers, so the helper receives each non-null pointer
/// minus 16 bytes. The associated globals are cleared after release. Cdecl,
/// with no arguments or declared result.
lf_checker_rt::export!(cdecl, rw_00b22d20() -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }

        const PORTAL_TABLE: u32 = 0x0165_7624;
        const PORTAL_COUNT: u32 = 0x0165_7628;
        const ROOM_POOL: u32 = 0x0104_578C;
        const PORTAL_POOL: u32 = 0x0104_5798;
        let table_pointer = read_u32(lf_checker_rt::relocated(PORTAL_TABLE) as u32);
        let _ = lf_checker_rt::callee_cdecl!(1, u32, table_pointer);
        write_u32(lf_checker_rt::relocated(PORTAL_TABLE) as u32, 0);
        write_u32(lf_checker_rt::relocated(PORTAL_COUNT) as u32, 0);

        let portal_pool = read_u32(lf_checker_rt::relocated(PORTAL_POOL) as u32);
        if portal_pool != 0 {
            let allocation_header = portal_pool.wrapping_sub(0x10);
            let _ = lf_checker_rt::callee_cdecl!(2, u32, allocation_header);
        }
        write_u32(lf_checker_rt::relocated(PORTAL_POOL) as u32, 0);

        let room_pool = read_u32(lf_checker_rt::relocated(ROOM_POOL) as u32);
        if room_pool != 0 {
            let allocation_header = room_pool.wrapping_sub(0x10);
            let _ = lf_checker_rt::callee_cdecl!(3, u32, allocation_header);
        }
        write_u32(lf_checker_rt::relocated(ROOM_POOL) as u32, 0);
        0
    }
});
