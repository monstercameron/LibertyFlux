// original: 0x00a1ac20 cam_lazy_handle_refresh (proposed)

/// Lazily creates a handle at `this + SLOT_OFF` and refreshes it.
///
/// `p` is `this + SLOT_OFF`. When `*p` is zero, the factory callee runs
/// and its answer is stored to `*p`; when that answer is nonzero the
/// handle is registered through the registrar callee (`*p` as its stack
/// argument, the factory answer as `ecx`). Then the gate callee runs:
/// when it approves and the global switch byte is zero, the factory runs
/// again and `*p` is overwritten unconditionally. Returns `*p`. (The two
/// factory calls and the gate call inherit whatever `ecx` the previous
/// stub left behind, so `ecx` is not compared on them; the registrar's
/// `ecx` is the scripted factory answer and is compared.)
///
/// Original: 0x00a1ac20 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a1ac20(this: u32) -> u32 {
    unsafe {
        const C_FACTORY: u32 = 1;
        const C_REGISTER: u32 = 2;
        const C_GATE: u32 = 3;
        const SLOT_OFF: u32 = 0x384;
        const SWITCH: u32 = 0x017f_5eb3;
        let p = this + SLOT_OFF;
        if ((p) as *const u32).read_unaligned() == 0 {
            let v = lf_checker_rt::callee_thiscall!(C_FACTORY, u32, this);
            (p as *mut u32).write_unaligned(v);
            if v != 0 {
                lf_checker_rt::callee_thiscall!(C_REGISTER, u32, v, p);
            }
        }
        let gate: u32 = lf_checker_rt::callee_thiscall!(C_GATE, u32, this);
        if gate as u8 != 0 && lf_checker_rt::global::<u8>(SWITCH).read() == 0 {
            let v = lf_checker_rt::callee_thiscall!(C_FACTORY, u32, this);
            (p as *mut u32).write_unaligned(v);
        }
        (p as *const u32).read_unaligned()
    }
});
