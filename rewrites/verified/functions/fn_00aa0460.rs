// original: 0x00aa0460 stream_notify_tagged (proposed)

/// Notify every list node whose tag word equals `tag`.
///
/// The node list hangs off the shared context (global at file address
/// 0x01bb6674, list head two levels down at `+0x26c` then `+0x0`; link at
/// node `+0`). A node whose word at `+0x80` equals `tag` is notified
/// through the intercepted routine with `node + 0x10` in ECX. The result is
/// always 0 (the loop's terminal null link).
///
/// Original: 0x00aa0460 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00aa0460(tag: u32) -> u32 {
    unsafe {
        const CONTEXT_GLOBAL: u32 = 0x01bb6674;
        const HEAD_OFF: u32 = 0x26c;
        const TAG_OFF: u32 = 0x80;
        const NOTIFY_THIS_DELTA: u32 = 0x10;
        const NOTIFY: u32 = 1;
        let ctx = lf_checker_rt::global::<u32>(CONTEXT_GLOBAL).read_unaligned();
        let mut node = (((ctx + HEAD_OFF) as *const u32).read_unaligned() as *const u32)
            .read_unaligned();
        while node != 0 {
            let next = (node as *const u32).read_unaligned();
            if ((node + TAG_OFF) as *const u32).read_unaligned() == tag {
                lf_checker_rt::callee_thiscall!(NOTIFY, u32, node.wrapping_add(NOTIFY_THIS_DELTA));
            }
            node = next;
        }
        0
    }
});
