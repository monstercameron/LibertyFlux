// original: 0x008834a0 stream_req_drain_spin (proposed) — DEFERRED (see results.json)
/// Drain a request's direct queue, spinning until another thread clears the flag.
///
/// Raises the busy flag at `obj+0x32`, then, while the drain flag at
/// `obj+0x30` stays set, flushes word `+0x28` (intercepted callee 1, cdecl)
/// and re-checks the flag, then flushes word `+0x24` (callee 1 again),
/// rebinds words `+0x14`/`+0x18`/`+0x1c` (intercepted callee 2, thiscall:
/// object in `ecx`), and commits words `+0x24` and `+0x20` (intercepted
/// callee 3, cdecl, called twice), looping until the flag clears. Lowers the
/// busy flag on exit.
///
/// Deferred: the drain flag is only ever cleared by another thread — no
/// instruction in the loop body writes it — so single-threaded trials can
/// only ever take the early exit. The rewrite below is faithful but only its
/// early-exit path is checkable.
///
/// Original: cdecl, one stack argument, no return value.
lf_checker_rt::export!(cdecl, rw_008834a0(obj: u32) -> u32 {
    unsafe {
        const DRAIN_FLAG: u32 = 0x30;
        const BUSY_FLAG: u32 = 0x32;
        const FLUSH_CALLEE: u32 = 1;
        const BIND_CALLEE: u32 = 2;
        const COMMIT_CALLEE: u32 = 3;
        ((obj + BUSY_FLAG) as *mut u8).write(1);
        if ((obj + DRAIN_FLAG) as *const u8).read() != 0 {
            loop {
                let b = ((obj + 0x28) as *const u32).read_unaligned();
                let _: u32 = lf_checker_rt::callee_cdecl!(FLUSH_CALLEE, u32, b);
                if ((obj + DRAIN_FLAG) as *const u8).read() == 0 {
                    break;
                }
                let a = ((obj + 0x24) as *const u32).read_unaligned();
                let _: u32 = lf_checker_rt::callee_cdecl!(FLUSH_CALLEE, u32, a);
                let w14 = ((obj + 0x14) as *const u32).read_unaligned();
                let w18 = ((obj + 0x18) as *const u32).read_unaligned();
                let w1c = ((obj + 0x1c) as *const u32).read_unaligned();
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(BIND_CALLEE, u32, obj, w14, w18, w1c);
                let c1 = ((obj + 0x24) as *const u32).read_unaligned();
                let _: u32 = lf_checker_rt::callee_cdecl!(COMMIT_CALLEE, u32, c1);
                let c2 = ((obj + 0x20) as *const u32).read_unaligned();
                let _: u32 = lf_checker_rt::callee_cdecl!(COMMIT_CALLEE, u32, c2);
                if ((obj + DRAIN_FLAG) as *const u8).read() == 0 {
                    break;
                }
            }
        }
        ((obj + BUSY_FLAG) as *mut u8).write(0);
        0
    }
});
