// original: 0x00CB3E30 CTaskComplexMoveWaitForTraffic::vf18

/// Create a wait-for-traffic subtask through the task allocator.
///
/// `this` is unused; `ped` is the ped (also unused here). The global task
/// allocator is fetched and asked for a worker (callee 1); a null worker
/// means a null result. Otherwise a wait subtask is built on it (callee 2)
/// with timeout 2000 ms and approach speed 8.0, and returned.
///
/// Original: 0x00CB3E30 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB3E30(this: u32, ped: u32) -> u32 {
    unsafe {
        const ALLOCATOR_GLOBAL: u32 = 0x167e2a0;
        const GET_WORKER: u32 = 1;
        const MAKE_WAIT: u32 = 2;
        const TIMEOUT_MS: u32 = 0x7d0;
        const SPEED_8: u32 = 0x41000000;
        let _ = (this, ped);
        let alloc = (lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL)).read_unaligned();
        let worker: u32 = lf_checker_rt::callee_thiscall!(GET_WORKER, u32, alloc);
        if worker == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(MAKE_WAIT, u32, worker, TIMEOUT_MS, 0, 0, SPEED_8)
    }
});
