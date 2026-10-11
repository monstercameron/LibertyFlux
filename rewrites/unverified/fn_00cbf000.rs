// original: 0x00CBF000 ped_task_candidate_filter
/// Test the first candidate against an entity identifier, then optionally
/// test the second candidate unless its value is `u32::MAX`. On the first
/// match, call the task filter with the entity, candidate, threshold word,
/// and sentinel. Candidate identifiers and the threshold are forwarded as
/// raw 32-bit stack words; the outer function is cdecl with five arguments.
lf_checker_rt::export!(cdecl, rw_00cbf000(
    _unused: u32,
    first_candidate: u32,
    entity_id: u32,
    threshold_bits: u32,
    second_candidate: u32,
) -> u32 {
    const NO_CANDIDATE: u32 = u32::MAX;
    let first_result = lf_checker_rt::callee_cdecl!(1, u32, first_candidate, entity_id);
    let found = if first_result != 0 {
        true
    } else if second_candidate != NO_CANDIDATE {
        lf_checker_rt::callee_cdecl!(1, u32, second_candidate, entity_id) != 0
    } else {
        false
    };
    if found {
        lf_checker_rt::callee_thiscall!(
            2,
            u32,
            entity_id,
            first_candidate,
            entity_id,
            threshold_bits,
            NO_CANDIDATE,
        )
    } else {
        0
    }
});
