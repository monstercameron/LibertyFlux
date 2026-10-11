// original: 0x009B0DD0 audPoliceScanner::clear_report_slot

/// Clears an indexed report slot. The slot begins at `this + index * 0x1a0`;
/// its unsigned record count is at `+0x9c`. For each record, the two non-null
/// resource handles at `+0xa4` and `+0xa8` are released, then the word at
/// `+0xa0` is cleared. The slot status word at `+0x201` is cleared on both the
/// empty and nonempty paths.
lf_checker_rt::export!(thiscall, rw_009b0dd0(this: u32, slot_index: u32) -> u32 {
    const SLOT_STRIDE: u32 = 0x1a0;
    const RECORD_COUNT: u32 = 0x9c;
    const FIRST_RECORD: u32 = 0xa4;
    const RECORD_STRIDE: u32 = 0x14;
    const STATUS: u32 = 0x201;

    unsafe {
        let slot = this.wrapping_add(slot_index.wrapping_mul(SLOT_STRIDE));
        let record_count = ((slot + RECORD_COUNT) as *const u32).read_unaligned();
        for index in 0..record_count {
            let record = slot.wrapping_add(FIRST_RECORD).wrapping_add(index.wrapping_mul(RECORD_STRIDE));
            let first_handle = (record as *const u32).read_unaligned();
            if first_handle != 0 {
                let _ = lf_checker_rt::callee_thiscall!(1, u32, first_handle, 0);
            }
            let second_handle = (record.wrapping_add(4) as *const u32).read_unaligned();
            if second_handle != 0 {
                let _ = lf_checker_rt::callee_thiscall!(2, u32, second_handle, 0);
            }
            (record.wrapping_sub(4) as *mut u32).write_unaligned(0);
        }
        ((slot + STATUS) as *mut u16).write_unaligned(0);
        0
    }
});
