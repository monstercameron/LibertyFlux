// original: 0x00cbac10 ped_iterate_apply
/// Walk peds matching a query, applying -4.0 to type 0x4B ones (3 calls).
///
/// Loops (stdcall, one stack argument): runs the find callee with
/// (`[a0 + 0x78]`, 3, 2) and stops with 0 when it answers null; when the
/// found entry's word at `+0xC` equals `0x4B`, runs the apply callee with
/// it and `-4.0` bits; then runs the continue callee with the same object
/// and arguments and loops while it answers non-null. Every exit returns
/// 0. All callees are intercepted; the find/continue answers come from
/// per-call sequences so the loop runs a bounded trip count on both
/// sides. No reads or writes of its own.
lf_checker_rt::export!(stdcall, rw_00cbac10(a0: u32) -> u32 {
    unsafe {
        /// Query-object slot in the argument, and entry type slot.
        const QUERY_OFF: u32 = 0x78;
        const TYPE_OFF: u32 = 0xC;
        /// Entry type the apply callee handles.
        const APPLY_TYPE: u32 = 0x4B;
        /// Value applied (-4.0 bits).
        const APPLY_VALUE: u32 = 0xC0800000;
        const FIND: u32 = 1;
        const APPLY: u32 = 2;
        const CONTINUE: u32 = 3;
        loop {
            let o = ((a0 + QUERY_OFF) as *const u32).read_unaligned();
            let r: u32 = lf_checker_rt::callee_thiscall!(FIND, u32, o, 3, 2);
            if r == 0 {
                return 0;
            }
            let t = ((r + TYPE_OFF) as *const u32).read_unaligned();
            if t == APPLY_TYPE {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    APPLY, u32, r, APPLY_VALUE);
            }
            let o2 = ((a0 + QUERY_OFF) as *const u32).read_unaligned();
            let r2: u32 = lf_checker_rt::callee_thiscall!(
                CONTINUE, u32, o2, 3, 2);
            if r2 == 0 {
                return 0;
            }
        }
    }
});
