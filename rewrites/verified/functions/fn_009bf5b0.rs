// original: 0x009BF5B0 ped_task_select_lowest_deadline_record
/// Counts matching active records whose type at byte offset 0x18 is 4 and
/// whose key at 0x1c equals `task_id`. It chooses the record with the lowest
/// unsigned deadline at offset 0x24, keeping the first record on ties. A
/// scripted mode helper selects a threshold of 10 when its low byte is zero
/// and 2 otherwise. When the count reaches that threshold, the selected record
/// address is passed to the release helper. The routine has no semantic return.
lf_checker_rt::export!(cdecl, rw_009bf5b0(task_id: u32) -> () {
    const TABLE_BASE_VA: u32 = 0x0129_2450;
    const RECORD_STRIDE: u32 = 0xD0;
    const RECORD_COUNT: u32 = 32;
    const RECORD_STATE_OFFSET: u32 = 0x10;
    const RECORD_TYPE_OFFSET: u32 = 0x18;
    const RECORD_ID_OFFSET: u32 = 0x1C;
    const RECORD_DEADLINE_OFFSET: u32 = 0x24;
    const REQUIRED_TYPE: u32 = 4;
    const STANDARD_THRESHOLD: u32 = 10;
    const REDUCED_THRESHOLD: u32 = 2;

    let mut match_count = 0u32;
    let mut selected_index = u32::MAX;
    let mut selected_deadline = 0u32;

    for record_index in 0..RECORD_COUNT {
        let record = TABLE_BASE_VA.wrapping_add(record_index.wrapping_mul(RECORD_STRIDE));
        let state = unsafe { lf_checker_rt::global::<u32>(record + RECORD_STATE_OFFSET).read() };
        if state == 0 {
            continue;
        }
        let record_type = unsafe { lf_checker_rt::global::<u32>(record + RECORD_TYPE_OFFSET).read() };
        if record_type != REQUIRED_TYPE {
            continue;
        }
        let record_id = unsafe { lf_checker_rt::global::<u32>(record + RECORD_ID_OFFSET).read() };
        if record_id != task_id {
            continue;
        }

        match_count = match_count.wrapping_add(1);
        let deadline = unsafe { lf_checker_rt::global::<u32>(record + RECORD_DEADLINE_OFFSET).read() };
        if selected_index == u32::MAX || deadline < selected_deadline {
            selected_deadline = deadline;
            selected_index = record_index;
        }
    }

    let mode = lf_checker_rt::callee_cdecl!(1, u32,);
    let threshold = if (mode as u8) != 0 { REDUCED_THRESHOLD } else { STANDARD_THRESHOLD };
    if match_count >= threshold && selected_index != u32::MAX {
        let selected_record = TABLE_BASE_VA.wrapping_add(selected_index.wrapping_mul(RECORD_STRIDE));
        let _ = lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(selected_record));
    }
});
