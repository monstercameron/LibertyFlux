// original: 0x00872560 trace_gate_0
//! Forward a trace record when the gate word at `a0+4` is non-negative:
//! pass the second argument plus a pointer to the overwritten first-arg
//! slot to the trace sink (callee 1). No return value is produced (EAX
//! keeps its entry value on the gated-out path, so the contract compares
//! no return channel), and the incoming stack is clobbered as scratch.
export!(cdecl, rw_00872560(a0: u32, a1: u32, a2: u32, _a3: u32, _a4: u32) -> u32 {
    unsafe {
        if *((a0.wrapping_add(4)) as *const i32) < 0 {
            return 0;
        }
        let mut slot: u32 = &a2 as *const u32 as u32;
        callee_stdcall!(1, u32, a1, &mut slot as *mut u32 as u32);
        0
    }
});
