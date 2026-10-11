// original: 0x00C4EC10 map_task_stream_status

/// Write the base task code and a status code selected by readiness and state.
///
/// The first stack argument is passed in ECX to a readiness helper. The next
/// two arguments are output pointers and the last is a state word. Both
/// outputs are initialized before the helper result is used. A true readiness
/// result writes `0xe5`; otherwise states 0, 1, 5, 6, 3 and 4 map to
/// `0xe3`, `0xe4`, `0xe8`, `0xe7`, `0xe1` and `0xe6`; other states keep `0xe2`.
lf_checker_rt::export!(cdecl, rw_00c4ec10(context: u32, base_code: u32, state_code: u32, state: u32) -> () {
    const READY_HELPER: u32 = 0;
    unsafe {
        (base_code as *mut u32).write_unaligned(0x1d);
        (state_code as *mut u32).write_unaligned(0xe2);
    }
    let ready = unsafe { lf_checker_rt::callee_thiscall!(READY_HELPER, u32, context) };
    if ready & 0xff != 0 {
        unsafe { (state_code as *mut u32).write_unaligned(0xe5); }
        return;
    }
    let mapped = match state {
        0 => 0xe3, 1 => 0xe4, 5 => 0xe8, 6 => 0xe7, 3 => 0xe1, 4 => 0xe6,
        _ => 0xe2,
    };
    unsafe { (state_code as *mut u32).write_unaligned(mapped); }
});
