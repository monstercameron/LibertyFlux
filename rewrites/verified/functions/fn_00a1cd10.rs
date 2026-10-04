// original: 0x00a1cd10 cam_target_worker_chain (proposed)

/// Validates a target pair and runs the worker/sink chain on it.
///
/// `a1` selects the subject for the predicate and worker callees, `a0`
/// points at a record with a linked state pointer at `+LINK_OFF` (a
/// record whose byte at `+LINK_FLAG_OFF` is set vetoes the run), and
/// `a2`/`a3` pass through to the worker. The chain runs only when the
/// predicate accepts `a1`, the veto is absent and the lookup callee on
/// `a1` returns nonzero; then the worker runs on
/// (`a1`, `a0`, `this+W0_OFF`, `this+W1_OFF`, `a2`, `a3`) and its answer
/// feeds the sink callee. Returns nothing.
///
/// Original: 0x00a1cd10 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00a1cd10(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const C_PRED: u32 = 1;
        const C_LOOKUP: u32 = 2;
        const C_WORKER: u32 = 3;
        const C_SINK: u32 = 4;
        const LINK_OFF: u32 = 0x6c;
        const LINK_FLAG_OFF: u32 = 0xe;
        const W0_OFF: u32 = 0x304;
        const W1_OFF: u32 = 0x308;
        let pred: u32 = lf_checker_rt::callee_thiscall!(C_PRED, u32, a1);
        if pred as u8 == 0 {
            return 0;
        }
        let link = ((a0 + LINK_OFF) as *const u32).read_unaligned();
        if link != 0 && ((link + LINK_FLAG_OFF) as *const u8).read() != 0 {
            return 0;
        }
        let t = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, a1);
        if t == 0 {
            return 0;
        }
        let u = lf_checker_rt::callee_thiscall!(
            C_WORKER, u32, a1, a0, this + W0_OFF, this + W1_OFF, a2, a3
        );
        lf_checker_rt::callee_thiscall!(C_SINK, u32, u);
        0
    }
});
