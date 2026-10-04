// original: 0x009fa550 playstat_stat_slots_reset
/// Reset the eight inline stat slots and the trailing counters.
///
/// Zeroes eight 32-byte records at the object base, stamps the constant
/// value 2 into the leading word of each record's two halves, clears three
/// trailing counter dwords and clears bit 0 of the flag byte. Returns the
/// object pointer.
export!(thiscall, rw_009fa550(this_ptr: u32) -> u32 {
    unsafe {
        const RECORDS: usize = 8;
        const RECORD_SIZE: usize = 0x20;
        const STAMP: u16 = 2;
        let base = this_ptr as *mut u8;
        for i in 0..RECORDS {
            let rec = base.add(i * RECORD_SIZE);
            core::ptr::write_bytes(rec, 0, RECORD_SIZE);
            (rec as *mut u16).write(STAMP);
            (rec.add(0x10) as *mut u16).write(STAMP);
        }
        (base.add(0x100) as *mut u32).write(0);
        (base.add(0x114) as *mut u32).write(0);
        (base.add(0x118) as *mut u32).write(0);
        let flag = base.add(0x11c);
        flag.write(flag.read() & !1);
        this_ptr
    }
});
