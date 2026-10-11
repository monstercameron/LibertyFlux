// original: 0x009B38E0 audPoliceScanner::tally_crime

/// Records a crime type in the 32-entry history at `this + 0x3470`. The
/// unsigned type value must be at most 15. A matching active record with a
/// nonzero count is incremented; otherwise the last inactive record is
/// initialized with count one, subtype zero, the type, and the current game
/// time. If no record is available, the object is unchanged.
lf_checker_rt::export!(thiscall, rw_009b38e0(this: u32, crime_type: u32) -> u32 {
    const HISTORY: u32 = 0x3470;
    const ENTRY_STRIDE: u32 = 0x14;
    const ENTRY_COUNT: u32 = 32;
    const ACTIVE: u32 = 0x10;
    const COUNT: u32 = 0x04;
    const SUBTYPE: u32 = 0x08;
    const TYPE: u32 = 0x00;
    const TIME: u32 = 0x0c;
    const GAME_TIME: u32 = 0x011735b4;

    if crime_type > 15 {
        return 0;
    }

    unsafe {
        let mut last_inactive = None;
        for index in 0..ENTRY_COUNT {
            let record = this.wrapping_add(HISTORY).wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            let active = ((record + ACTIVE) as *const u8).read();
            if active == 0 {
                last_inactive = Some(index);
                continue;
            }

            let stored_type = (record as *const u32).read_unaligned();
            let count = ((record + COUNT) as *const u32).read_unaligned();
            if stored_type == crime_type && count != 0 {
                ((record + COUNT) as *mut u32).write_unaligned(count.wrapping_add(1));
                return 0;
            }
        }

        if let Some(index) = last_inactive {
            let record = this.wrapping_add(HISTORY).wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            ((record + ACTIVE) as *mut u8).write(1);
            ((record + COUNT) as *mut u32).write_unaligned(1);
            ((record + SUBTYPE) as *mut u32).write_unaligned(0);
            (record as *mut u32).write_unaligned(crime_type);
            let now = lf_checker_rt::global::<u32>(GAME_TIME).read_unaligned();
            ((record + TIME) as *mut u32).write_unaligned(now);
        }
        0
    }
});
