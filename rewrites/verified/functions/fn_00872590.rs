// original: 0x00872590 trace_gate_1
//! Forward a trace record when the gate word at `a0+4` is at least 1.
//! Same shape as the threshold-0 gate; see there for the contract notes.
export!(cdecl, rw_00872590(a0: u32, a1: u32, a2: u32, _a3: u32, _a4: u32) -> u32 {
    unsafe {
        if *((a0.wrapping_add(4)) as *const i32) < 1 {
            return 0;
        }
        let mut slot: u32 = &a2 as *const u32 as u32;
        callee_stdcall!(1, u32, a1, &mut slot as *mut u32 as u32);
        0
    }
});
