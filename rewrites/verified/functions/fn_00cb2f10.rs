// original: 0x00CB2F10 CTaskComplexMoveWaitForTraffic::vf19

/// Start or keep waiting for traffic to clear.
///
/// `this` is the complex task, `ped` the ped. The current clock (global
/// 0x11735b4) seeds the wait deadline (`this+0x20`), the span is fixed at
/// 10000 ms (`this+0x24`) and the waiting flag (`this+0x28`) is set; then
/// the traffic probe (callee 1) runs. When it reports traffic, the hold
/// flag (`this+0x60`) is set and the result is null. Otherwise a worker is
/// fetched from the global allocator (callee 2) and a timed wait subtask
/// (5000 ms, speed 8.0) is built on it (callee 3); a missing worker
/// yields null.
///
/// Original: 0x00CB2F10 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB2F10(this: u32, ped: u32) -> u32 {
    unsafe {
        const TRAFFIC_PROBE: u32 = 1;
        const GET_WORKER: u32 = 2;
        const MAKE_WAIT: u32 = 3;
        const CLOCK_GLOBAL: u32 = 0x11735b4;
        const ALLOCATOR_GLOBAL: u32 = 0x167e2a0;
        const DEADLINE: u32 = 0x20;
        const SPAN: u32 = 0x24;
        const SPAN_MS: u32 = 0x2710;
        const WAITING: u32 = 0x28;
        const HOLD: u32 = 0x60;
        const TIMEOUT_MS: u32 = 0x1388;
        const SPEED_8: u32 = 0x41000000;
        let now = (lf_checker_rt::global::<u32>(CLOCK_GLOBAL)).read_unaligned();
        (this.wrapping_add(DEADLINE) as *mut u32).write_unaligned(now);
        (this.wrapping_add(SPAN) as *mut u32).write_unaligned(SPAN_MS);
        (this.wrapping_add(WAITING) as *mut u8).write_unaligned(1);
        if lf_checker_rt::callee_thiscall!(TRAFFIC_PROBE, u32, this, ped) as u8 != 0 {
            (this.wrapping_add(HOLD) as *mut u8).write_unaligned(1);
            return 0;
        }
        let alloc = (lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL)).read_unaligned();
        let worker: u32 = lf_checker_rt::callee_thiscall!(GET_WORKER, u32, alloc);
        if worker == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(MAKE_WAIT, u32, worker, TIMEOUT_MS, 0, 0, SPEED_8)
    }
});
