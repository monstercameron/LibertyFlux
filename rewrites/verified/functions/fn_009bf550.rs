// original: 0x009BF550 ped_task_refresh_matching_deadlines
/// Scans 32 global task records at a 0xd0-byte stride for the supplied task
/// id. A record must have a nonzero state at offset 0x10 and type 4 at 0x18.
/// If its deadline at 0x24 is more than 1000 unsigned ticks beyond the saved
/// current time, calls the delay helper and stores current time plus its
/// result. The helper call is scripted by the checker.
lf_checker_rt::export!(cdecl, rw_009bf550(task_id: u32) -> () {
    const TABLE_FIRST_ID_VA: u32 = 0x0129_246C;
    const CURRENT_TIME_VA: u32 = 0x0117_35B4;
    const RECORD_STRIDE: u32 = 0xD0;
    const RECORD_COUNT: u32 = 32;
    const STATE_OFFSET_FROM_ID: u32 = 0x0C;
    const TYPE_OFFSET_FROM_ID: u32 = 4;
    const DEADLINE_OFFSET_FROM_ID: u32 = 8;
    const REQUIRED_TYPE: u32 = 4;
    const MINIMUM_DELAY_GAP: u32 = 1000;

    let current_time = unsafe { lf_checker_rt::global::<u32>(CURRENT_TIME_VA).read() };
    for record_index in 0..RECORD_COUNT {
        let id_va = TABLE_FIRST_ID_VA.wrapping_add(record_index.wrapping_mul(RECORD_STRIDE));
        let state = unsafe { lf_checker_rt::global::<u32>(id_va - STATE_OFFSET_FROM_ID).read() };
        if state == 0 {
            continue;
        }
        let record_id = unsafe { lf_checker_rt::global::<u32>(id_va).read() };
        if record_id != task_id {
            continue;
        }
        let record_type = unsafe { lf_checker_rt::global::<u32>(id_va - TYPE_OFFSET_FROM_ID).read() };
        if record_type != REQUIRED_TYPE {
            continue;
        }

        let deadline_va = id_va.wrapping_add(DEADLINE_OFFSET_FROM_ID);
        let deadline = unsafe { lf_checker_rt::global::<u32>(deadline_va).read() };
        if deadline.wrapping_sub(current_time) > MINIMUM_DELAY_GAP {
            let delay = lf_checker_rt::callee_cdecl!(1, u32, task_id);
            unsafe { lf_checker_rt::global::<u32>(deadline_va).write(current_time.wrapping_add(delay)) };
        }
    }
});
