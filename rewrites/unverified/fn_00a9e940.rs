// original: 0x00a9e940 stream_refresh_matching (proposed)

/// Refresh every list node that names `obj` and disagrees on the tag.
///
/// `obj` points to a record with a flag word at `+0x24`; when bit 26 is
/// clear there is nothing to do. Otherwise the node list hanging off
/// `this + 0x8ec50` is walked (link at node `+0`). A node is refreshed
/// (through the intercepted per-node routine with the node in ECX) when it
/// names `obj` at `+0x68` but holds a different tag than `tag` at `+0x08`.
///
/// Original: 0x00a9e940 (thiscall, two stack words; no result).
lf_checker_rt::export!(thiscall, rw_00a9e940(this: u32, obj: u32, tag: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x24;
        const GATE_BIT: u32 = 1 << 26;
        const LIST_OFF: u32 = 0x8ec50;
        const NAME_OFF: u32 = 0x68;
        const TAG_OFF: u32 = 8;
        const REFRESH: u32 = 1;
        if ((obj + FLAG_OFF) as *const u32).read_unaligned() & GATE_BIT == 0 {
            return 0;
        }
        let mut node = ((this + LIST_OFF) as *const u32).read_unaligned();
        while node != 0 {
            let names = ((node + NAME_OFF) as *const u32).read_unaligned() == obj;
            let differs = ((node + TAG_OFF) as *const u32).read_unaligned() != tag;
            if names && differs {
                lf_checker_rt::callee_thiscall!(REFRESH, u32, node);
            }
            node = (node as *const u32).read_unaligned();
        }
        0
    }
});
