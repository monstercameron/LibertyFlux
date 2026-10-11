// original: 0x009C0990 ped_task_event_flag_update
/// Scans 32 fixed-stride task records. For each record with a nonzero state
/// at byte offset 0x10 and a key at 0x18 equal to `event_id`, it stores the
/// low byte of `flag` at offset 0xcb. It returns the relocated address just
/// past the table, matching the original's final accumulator value.
lf_checker_rt::export!(cdecl, rw_009c0990(event_id: u32, flag: u32) -> u32 {
    const TABLE_BASE: u32 = 0x0129_2450;
    const RECORD_STRIDE: u32 = 0xD0;
    const RECORD_COUNT: u32 = 32;

    for record_index in 0..RECORD_COUNT {
        let record = TABLE_BASE.wrapping_add(record_index.wrapping_mul(RECORD_STRIDE));
        let state = unsafe { lf_checker_rt::global::<u32>(record + 0x10).read() };
        let key = unsafe { lf_checker_rt::global::<u32>(record + 0x18).read() };
        if state != 0 && key == event_id {
            unsafe { lf_checker_rt::global::<u8>(record + 0xCB).write(flag as u8) };
        }
    }
    lf_checker_rt::relocated(0x0129_3E68)
});
