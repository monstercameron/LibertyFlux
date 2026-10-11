// original: 0x009B3CB0 audPoliceScanner::tally_crime_detailed

/// Records a crime type and detail class in the same 32-entry history as the
/// basic tally helper. The type is an unsigned value limited to 0 through 15.
/// The low byte of `detailed` selects subtype one when zero and subtype three
/// for every nonzero value. Matching active records with a nonzero count are
/// incremented; otherwise the first inactive record is initialized.
lf_checker_rt::export!(thiscall, rw_009b3cb0(this: u32, crime_type: u32, detailed: u32) -> u32 {
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
    let subtype = if detailed as u8 == 0 { 1 } else { 3 };

    unsafe {
        let mut first_inactive = u32::MAX;
        for index in 0..ENTRY_COUNT {
            let record = this.wrapping_add(HISTORY).wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            let active = ((record + ACTIVE) as *const u8).read();
            if active == 0 {
                if first_inactive == u32::MAX {
                    first_inactive = index;
                }
                continue;
            }

            let stored_type = (record as *const u32).read_unaligned();
            let stored_subtype = ((record + SUBTYPE) as *const u32).read_unaligned();
            if stored_type == crime_type && stored_subtype == subtype {
                let count = ((record + COUNT) as *const u32).read_unaligned();
                if count != 0 {
                    ((record + COUNT) as *mut u32).write_unaligned(count.wrapping_add(1));
                    return 0;
                }
            }
        }

        if first_inactive != u32::MAX {
            let record = this.wrapping_add(HISTORY).wrapping_add(first_inactive.wrapping_mul(ENTRY_STRIDE));
            ((record + ACTIVE) as *mut u8).write(1);
            ((record + COUNT) as *mut u32).write_unaligned(1);
            ((record + SUBTYPE) as *mut u32).write_unaligned(subtype);
            (record as *mut u32).write_unaligned(crime_type);
            let now = lf_checker_rt::global::<u32>(GAME_TIME).read_unaligned();
            ((record + TIME) as *mut u32).write_unaligned(now);
        }
        0
    }
});
