// original: 0x00B3DAF0 peds_tasks_cluster_a (proposed)
//
// PROBE STUB. The original reads globals through unrelocated absolute
// addresses from its fifth instruction on, which faults under the checker
// worker on this machine (abs_shadow unavailable). This stub exists only so
// the probe trial can run the rewrite side while the original side faults.

lf_checker_rt::export!(cdecl, rw_00B3DAF0(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    let _ = (a0, a1, a2, a3);
    0
});
