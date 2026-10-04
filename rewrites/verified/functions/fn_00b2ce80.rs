// original: 0x00b2ce80 garage_record_reset
// 0xB2CE80 garage_record_reset (thiscall/0).
//
// Clears the status flag bit, stamps the slot empty (-1) with a zero timer,
// and parks the sub-state at 0, or 1 for records of kind 2.
export!(thiscall, rw_00b2ce80(rec: *mut u8) -> () {
    unsafe {
        let kind = *rec.add(0x48);
        *rec.add(0x4C) &= 0xEF;
        *rec.add(0x49) = 0;
        *(rec.add(0x38) as *mut u32) = 0xFFFF_FFFF;
        *(rec.add(0x44) as *mut u32) = 0;
        if kind == 2 {
            *rec.add(0x49) = 1;
        }
    }
});
