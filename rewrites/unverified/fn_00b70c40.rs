// original: 0x00b70c40 task_gate_on_state_and_subcheck (proposed)
/// Gate on a state byte and a helper result.
///
/// `this` points to the task. When the state byte at `+0x99` is below 4 the
/// function returns 0 at once. Otherwise it runs the helper (callee 1,
/// thiscall/0, `this` in ECX) and returns 1 only when the helper answers 2.
///
/// Only AL is meaningful: the upper bytes of EAX keep whatever the helper
/// (or, on the early path, the caller) left there, so the contract compares
/// `al`.
///
/// Original: 0x00b70c40 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00b70c40(this: u32) -> u32 {
    unsafe {
        const STATE_OFF: u32 = 0x99;
        const STATE_MIN: u8 = 4;
        const HELPER_OK: u32 = 2;
        let st = ((this + STATE_OFF) as *const u8).read();
        if st < STATE_MIN {
            return 0;
        }
        let ans: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
        (ans == HELPER_OK) as u32
    }
});

