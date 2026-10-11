// original: 0x005E85D0 CDispatchOrderArrayHandler::vf22

/// Advances a signed dispatch cursor by 50 when the result remains
/// nonnegative and no greater than the selected signed bound. The state
/// pointer is the first of two stack arguments; its cursor is at byte offset
/// `0x10`, the selector byte at `0x1c`, and the bounds at `0x0c` and `0x14`.
/// Selector bit zero chooses the first bound. ECX is unused, EAX returns the
/// state pointer, and the thiscall method removes both stack arguments.
lf_checker_rt::export!(thiscall, rw_005e85d0(_this: u32, state: u32, _unused: u32) -> u32 {
    const CURSOR: u32 = 0x10;
    const SELECTOR: u32 = 0x1c;
    const BOUND_A: u32 = 0x0c;
    const BOUND_B: u32 = 0x14;
    const STEP: u32 = 0x32;

    unsafe {
        let current = (state.wrapping_add(CURSOR) as *const u32).read_unaligned();
        let next = current.wrapping_add(STEP);
        let flags = (state.wrapping_add(SELECTOR) as *const u8).read();
        let bound_offset = if flags & 1 != 0 { BOUND_A } else { BOUND_B };
        let bound = (state.wrapping_add(bound_offset) as *const u32).read_unaligned();
        if (next as i32) >= 0 && (next as i32) <= (bound as i32) {
            (state.wrapping_add(CURSOR) as *mut u32).write_unaligned(next);
        }
    }
    state
});
