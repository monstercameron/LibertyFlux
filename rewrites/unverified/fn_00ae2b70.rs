// original: 0x00ae2b70 input_ui_list_worker

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

// Shared gate byte (see fn_00ae2a90 contract for the full table).
const B_GATE: u32 = 0x1593312; // early-out gate byte

fn null_fault() -> u32 {
    unsafe { (0 as *const u32).read_volatile() }
}

/// Table-list refresh worker, stage 1. When the shared gate byte is nonzero
/// the function forwards its four argument addresses plus a tag to the
/// notifier and returns its answer. That path is fully verified by the
/// stage-1 contract (gate pinned nonzero, all four forwarded values snapped).
///
/// The gate==0 fallthrough (row select, element loop, float scaling over
/// roughly 1800 bytes) is NOT covered yet: reaching it faults loudly via a
/// null read, so the stage-1 verdict cannot silently include it. See the
/// lane report for the remaining plan.
export!(cdecl, rw_00ae2b70(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    if unsafe { global::<u8>(B_GATE).read() } != 0 {
        let mut c0 = [a0];
        let mut c1 = [a1];
        let mut c2 = [a2];
        let mut c3 = [a3];
        return callee_cdecl!(
            1, u32, relocated(0xae9200),
            c0.as_mut_ptr() as u32, c1.as_mut_ptr() as u32,
            c2.as_mut_ptr() as u32, c3.as_mut_ptr() as u32
        );
    }
    // Stage 1 ends here: the pinned corpus never reaches this. A null read
    // faults the trial (like the original would diverge), never a silent pass.
    null_fault();
    0
});
