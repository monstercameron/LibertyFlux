// original: 0x008d5ef0 check_record_ready
// Reports whether a record is ready: null records and records failing the
// global gate or the link walk count as ready, otherwise a resolver chain
// decides. Only the low byte of the result is defined (the upper bits are
// leftovers), so the contract compares al only.
export!(cdecl, rw_008d5ef0(rec: *const u8) -> u32 {
    unsafe {
        if rec.is_null() {
            return 1;
        }
        if *global::<u32>(0x011F_7060) != 1
            && *global::<u32>(0x0120_88B4) == 0xFFFF_FFFF
            && *global::<u32>(0x0103_7720) != 0x12
            && *rec.wrapping_add(0x4B) & 0x40 != 0
        {
            return 1;
        }
        let b1 = *(rec as *const u32);
        if b1 == 0 {
            return 1;
        }
        let b2 = *((b1 as *const u32).wrapping_add(3));
        if b2 == 0 {
            return 1;
        }
        if *global::<u8>(0x012F_B1D4) == 0
            && *((b2 as *const u8).wrapping_add(0x24)) & 1 == 0
        {
            return 1;
        }
        let h: u32 = callee_cdecl!(1, u32, 0);
        if h == 0 {
            return 0;
        }
        if *global::<u8>(0x0103_2768) != 0 {
            let r: u32 = callee_cdecl!(2, u32, rec as u32, h);
            if r & 0xFF != 0 {
                return 1;
            }
        }
        let r: u32 = callee_cdecl!(3, u32, rec as u32, h);
        if r & 0xFF != 0 {
            return 1;
        }
        let r: u32 = callee_cdecl!(4, u32, rec as u32, h);
        if r & 0xFF != 0 {
            return 1;
        }
        if *global::<u8>(0x012F_B1D4) == 0 {
            return 0;
        }
        let d: u32 = callee_cdecl!(5, u32,);
        u32::from(b2 == d)
    }
});
