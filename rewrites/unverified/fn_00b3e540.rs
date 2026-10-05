// original: 0x00B3E540 peds_tasks_cluster_c (proposed)
//
// PROBE STUB. The original reads a global through an unrelocated absolute
// address in its second instruction, which faults under the checker worker
// on this machine (abs_shadow unavailable). This stub exists only so the
// probe trial can run the rewrite side while the original side faults.

lf_checker_rt::export!(stdcall, rw_00B3E540(a0: u32, a1: u32, a2: u32) -> u32 {
    let _ = (a0, a1, a2);
    0
});
