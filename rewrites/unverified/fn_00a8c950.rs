// original: 0x00a8c950 pool_collect_all (proposed)

/// Collect every indexed table row into `head`.
///
/// `this` is the pool and `head` points at the list head word. The table
/// callee reports the row count (0 or 2 in the proof): each row and
/// sub-slot runs the cell callee, and non-null cells resolve to nodes
/// pushed onto `head` (node takes the cell, links the old head).
/// Returns the last callee answer. No enable gate: the count alone
/// decides.
///
/// Original: 0x00A8C950 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8c950(this: u32, head: u32) -> u32 {
    unsafe {
        const CALLEE_RESOLVE: u32 = 2;
        const CALLEE_ROWS: u32 = 3;
        const CALLEE_CELL: u32 = 4;
        const RESOLVER: u32 = 0x12b4164;
        const NODE_LINK: u32 = 4;
        const SUB_SLOTS: u32 = 5;
        let rows = lf_checker_rt::callee_thiscall!(
            CALLEE_ROWS,
            u32,
            this,
            0
        );
        let mut last = rows;
        if (rows as i32) > 0 {
            let mut row = 0u32;
            while (row as i32) < (rows as i32) {
                let mut sub = 0u32;
                while sub < SUB_SLOTS {
                    let cell = lf_checker_rt::callee_thiscall!(
                        CALLEE_CELL,
                        u32,
                        this,
                        0,
                        row,
                        sub
                    );
                    last = cell;
                    if cell != 0 {
                        let resolver = lf_checker_rt::global::<u32>(RESOLVER)
                            .read_unaligned();
                        let node = lf_checker_rt::callee_thiscall!(
                            CALLEE_RESOLVE,
                            u32,
                            resolver
                        );
                        if node != 0 {
                            (node as *mut u32).write_unaligned(cell);
                        }
                        let old =
                            (head as *const u32).read_unaligned();
                        ((node + NODE_LINK) as *mut u32)
                            .write_unaligned(old);
                        (head as *mut u32).write_unaligned(node);
                        last = node;
                    }
                    sub += 1;
                }
                row += 1;
            }
        }
        last
    }
});
