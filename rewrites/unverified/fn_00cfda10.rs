// original: 0x00cfda10 task_state_copy_if_active (proposed)

/// Copies a 16-byte animation-state record to the caller's buffer when the
/// task's state word says it is active.
///
/// `this` (ECX) is the task object. The dword at `this + 0x54` is a state
/// code: only 1, 2, 3 or 4 mean "active" (compared in that order, each
/// equality branching to the copy). When active, four dwords are copied from
/// `this + 0x60`, `+0x64`, `+0x68`, `+0x6c` to `out + 0`, `+4`, `+8`, `+12`;
/// the middle two are floats moved with SSE but the move is a pure bit copy.
/// Otherwise nothing is read past the state word and nothing is written.
/// No return value.
///
/// Original: 0x00cfda10 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00cfda10(this: u32, out: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x54;
        const SRC: u32 = 0x60;
        let state = (this.wrapping_add(STATE) as *const u32).read_unaligned();
        if state == 1 || state == 2 || state == 3 || state == 4 {
            for i in 0..4u32 {
                let w = (this.wrapping_add(SRC + i * 4) as *const u32).read_unaligned();
                (out.wrapping_add(i * 4) as *mut u32).write_unaligned(w);
            }
        }
        0
    }
});
