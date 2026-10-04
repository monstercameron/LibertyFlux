// original: 0x009FD270 frag_space_notify (proposed)

/// Notify every watcher in the global frag space whose pair matches `arg`.
///
/// Walks the global space chain to a counted array of node pointers and, for
/// each non-null node, either checks the node's own pair (`+0x34`/`+0x38`)
/// when its sub-count (`+0x7c`) is zero or below, or checks each of its
/// `sub - 1` sub-entries at `+0x28c` when above 1 (a sub-count of exactly 1
/// checks nothing). Each entry whose pair contains `arg` is passed to the
/// notify callee. Returns nothing.
///
/// Original: 0x009FD270 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_009FD270(arg: u32) -> u32 {
    unsafe {
        const SPACE: u32 = 0x018B896C;
        const INNER_OFF: u32 = 0x9C;
        const LIST_OFF: u32 = 4;
        const COUNT_OFF: u32 = 8;
        const SUB_OFF: u32 = 0x7C;
        const PAIR_A: u32 = 0x34;
        const PAIR_B: u32 = 0x38;
        const SUBS_OFF: u32 = 0x28C;
        const ENTRY_STRIDE: u32 = 8;
        let g0 = (lf_checker_rt::relocated(SPACE) as *const u32).read_unaligned();
        let g1 = ((g0 + INNER_OFF) as *const u32).read_unaligned();
        let g2 = ((g1 + LIST_OFF) as *const u32).read_unaligned();
        let count = ((g2 + COUNT_OFF) as *const i32).read_unaligned();
        if count <= 0 {
            return 0;
        }
        let mut p = ((g2 + LIST_OFF) as *const u32).read_unaligned() + 4;
        let mut i = 0;
        while i < count {
            let node = (p as *const u32).read_unaligned();
            p += ENTRY_STRIDE;
            i += 1;
            if node == 0 {
                continue;
            }
            let sub = ((node + SUB_OFF) as *const i32).read_unaligned();
            if sub <= 0 {
                if ((node + PAIR_A) as *const u32).read_unaligned() == arg
                    || ((node + PAIR_B) as *const u32).read_unaligned() == arg
                {
                    lf_checker_rt::callee_thiscall!(1, u32, node);
                }
            } else if sub > 1 {
                let mut k = 0;
                while k < sub - 1 {
                    let e = (((node + SUBS_OFF) as *const u32).add(k as usize)).read_unaligned();
                    if ((e + PAIR_A) as *const u32).read_unaligned() == arg
                        || ((e + PAIR_B) as *const u32).read_unaligned() == arg
                    {
                        lf_checker_rt::callee_thiscall!(1, u32, e);
                    }
                    k += 1;
                }
            }
        }
        0
    }
});
