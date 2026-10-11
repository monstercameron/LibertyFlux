// original: 0x00B37EC0 ped_task_candidate_filter_stage

/// Entry-path stage for the two-list candidate filter. With both global
/// lists empty, the original reaches the candidate-selection gate with a
/// zero count and calls it with ECX equal to zero; the gate's answer cannot
/// select an item because the count is below the later minimum. The function
/// then checks its stack cookie. The non-empty list scans, candidate ranking,
/// callbacks, and selected-item effects are not implemented by this stage.
/// The original routine is cdecl with a target pointer and a count cap on the
/// stack. The stage intentionally compares only the empty-list path.
lf_checker_rt::export!(cdecl, rw_00b37ec0(_target_xy: u32, _max_count: u32) -> u32 {
    unsafe { run_empty_lists_00b37ec0(false) }
});

const LIST_PRIMARY_00B37EC0: u32 = 0x018B_6F1C;
const LIST_SECONDARY_00B37EC0: u32 = 0x018B_6F10;

#[inline(always)]
unsafe fn read_u32_00b37ec0(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

unsafe fn run_empty_lists_00b37ec0(mutant: bool) -> u32 {
    unsafe {
        let primary = read_u32_00b37ec0(lf_checker_rt::global::<u32>(LIST_PRIMARY_00B37EC0) as u32);
        let secondary = read_u32_00b37ec0(lf_checker_rt::global::<u32>(LIST_SECONDARY_00B37EC0) as u32);
        if read_u32_00b37ec0(primary + 8) != 0 || read_u32_00b37ec0(secondary + 8) != 0 {
            return 0;
        }

        let gate_this = if mutant { 1 } else { 0 };
        let _gate_answer = lf_checker_rt::callee_thiscall!(1, u32, gate_this);
        let _cookie_answer = lf_checker_rt::callee_thiscall!(2, u32, 0u32);
        0
    }
}
