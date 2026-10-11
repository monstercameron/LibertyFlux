// original: 0x00d17960 clear_task_group_records
/// Clears the task's embedded primary record, then clears each counted record
/// in four arrays. Every count uses a signed greater-than-zero loop bound;
/// nonpositive counts skip the associated array. The record cleanup is an
/// intercepted thiscall for each visited record.

const GROUP_A_COUNT: u32 = 0x13A0;
const GROUP_A_START: u32 = 0x90;
const GROUP_A_STRIDE: u32 = 0xB0;
const GROUP_B_COUNT: u32 = 0x13A4;
const GROUP_B_START: u32 = 0x820;
const GROUP_B_STRIDE: u32 = 0xC0;
const GROUP_C_COUNT: u32 = 0x13A8;
const GROUP_C_START: u32 = 0x8E0;
const GROUP_C_STRIDE: u32 = 0xE0;
const GROUP_D_COUNT: u32 = 0x13AC;
const GROUP_D_START: u32 = 0x1280;
const GROUP_D_STRIDE: u32 = 0x120;

#[inline(always)]
unsafe fn read_i32(address: u32) -> i32 {
    (address as *const i32).read_unaligned()
}

#[inline(always)]
unsafe fn clear_group(task: u32, count_offset: u32, records_offset: u32, stride: u32) {
    let count = read_i32(task + count_offset);
    if count > 0 {
        for index in 0..count as u32 {
            let record = task.wrapping_add(records_offset).wrapping_add(index.wrapping_mul(stride));
            let _result = lf_checker_rt::callee_thiscall!(1, u32, record);
        }
    }
}

lf_checker_rt::export!(thiscall, rw_00d17960(task: u32) -> u32 {
    unsafe {
        let _result = lf_checker_rt::callee_thiscall!(1, u32, task);
        clear_group(task, GROUP_B_COUNT, GROUP_B_START, GROUP_B_STRIDE);
        clear_group(task, GROUP_A_COUNT, GROUP_A_START, GROUP_A_STRIDE);
        clear_group(task, GROUP_C_COUNT, GROUP_C_START, GROUP_C_STRIDE);
        clear_group(task, GROUP_D_COUNT, GROUP_D_START, GROUP_D_STRIDE);
        0
    }
});
