// original: 0x009B09E0 audPoliceScanner::on_wanted_level_changed

/// Updates the wanted-level state. State two sets the notification byte at
/// `+0x36f1`; state one releases the indexed report slot and then resets the
/// state word at `+0x4c`. Other values leave the object unchanged.
lf_checker_rt::export!(thiscall, rw_009b09e0(this: u32) -> u32 {
    const STATE: u32 = 0x4c;
    const REPORT_INDEX: u32 = 0x30;
    const NOTIFY_PENDING: u32 = 0x36f1;

    unsafe {
        let state = ((this + STATE) as *const u32).read_unaligned();
        let mut result = state;
        if state == 2 {
            ((this + NOTIFY_PENDING) as *mut u8).write(1);
        } else if state == 1 {
            let report_index = ((this + REPORT_INDEX) as *const u32).read_unaligned();
            result = lf_checker_rt::callee_thiscall!(1, u32, this, report_index);
            ((this + STATE) as *mut u32).write_unaligned(0);
        }
        result
    }
});
