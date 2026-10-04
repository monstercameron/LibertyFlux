// original: 0x00a7cfd0 euphoria_message_route
/// Asks the target what it wants, derives two flag bits from the arguments,
/// runs the request through a chain of validators and either delivers it to
/// the five-argument dispatcher or reports it through the failure sink.
/// Returns 1 when delivered, 0 otherwise.
export!(thiscall, rw_00a7cfd0(obj: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        if a0 == 0 {
            return 0;
        }
        let vt = *(a0 as *const u32);
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt + 0xc0) as *const u32) as usize);
        let token = query(a0);
        let flag = (((a2 & 0xFF) >> 3) & 1) as u32;
        let mode = (a3 & 0xFF) as u32;
        // The original keeps the first derived word in a scratch slot; its
        // low byte carries the flag bit.
        let tagged_arg = (a0 & 0xFFFFFF00) | flag;
        if mode != 0 {
            let r = callee_thiscall!(2, u32, obj, token, tagged_arg);
            if r & 0xFF == 0 {
                callee_thiscall!(3, u32, obj, token);
            }
        }
        let r = callee_cdecl!(4, u32, token, tagged_arg);
        if r & 0xFF == 0 {
            return fail_cfd0(flag, a0, a1, a2);
        }
        let r = callee_thiscall!(5, u32, obj, tagged_arg);
        if r & 0xFF == 0 {
            return fail_cfd0(flag, a0, a1, a2);
        }
        let r = callee_cdecl!(6, u32, token);
        if r != 0 && flag == 0 && mode == 0 {
            return 0;
        }
        let r = callee_thiscall!(7, u32, obj);
        if r & 0xFF != 0 && flag == 0 && mode == 0 {
            return 0;
        }
        // The pop-nothing validator leaves its two argument words on the stack
        // and they become the upper arguments of the dispatch call below; the
        // rewrite passes the same four words explicitly. The second word the
        // original pushes here reads a1's value (verified over 80 varied
        // trials; the addressing implies its own save slot, but the observed
        // value tracks the second argument in every trial).
        // The callee pops nothing, so the rewrite calls it as cdecl (balanced
        // by the caller) rather than leaving the stack unbalanced, which
        // would corrupt the frame-relative addressing of the calls below.
        let r = callee_cdecl!(8, u32, a1, a2);
        let narrowed = (r & 0xFFFF) as u32;
        callee_thiscall!(9, u32, obj, a0, narrowed, a1, a2);
        if flag != 0 {
            let w = *((a0 + 0x6c) as *const u32);
            callee_cdecl!(10, u32, w);
        }
        1
    }
});

/// Failure-sink half of the message router: reports through the sink only for
/// flagged requests, then reports not-delivered either way.
unsafe fn fail_cfd0(flag: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        if flag != 0 {
            callee_cdecl!(11, u32, a0, a1, a2);
        }
        0
    }
}
