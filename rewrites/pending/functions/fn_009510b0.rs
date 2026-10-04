// original: 0x009510b0 clear_records_until_empty
/// Drains the record source, clearing each record to the empty pattern.
///
/// Repeatedly fetches the next record from the shared source; every live one
/// is reset to two zero words, a 0xffffffff marker and a zero byte. Stops at
/// the first null and returns 0.
export!(cdecl, rw_009510b0() -> u32 {
    unsafe {
        const SOURCE: u32 = 0x011F_A01C;
        const EMPTY_MARK: u32 = 0xFFFF_FFFF;
        loop {
            let rec = callee_thiscall!(1, u32, relocated(SOURCE));
            if rec == 0 {
                return 0;
            }
            *(rec as *mut u32) = 0;
            *((rec.wrapping_add(4)) as *mut u32) = 0;
            *((rec.wrapping_add(8)) as *mut u32) = EMPTY_MARK;
            *((rec.wrapping_add(12)) as *mut u8) = 0;
        }
    }
});
