// original: 0x00B3E000 peds_tasks_cluster_b (proposed)
//
// PROBE STUB. The original reads globals through unrelocated absolute
// addresses from its fifth instruction on, which faults under the checker
// worker on this machine (abs_shadow unavailable). This stub exists only so
// the probe trial can run the rewrite side while the original side faults.

lf_checker_rt::export!(cdecl, rw_00B3E000(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    let _ = (a0, a1, a2, a3, a4, a5);
    0
});
