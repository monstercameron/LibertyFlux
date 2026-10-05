// original: 0x00a8c440 pool_collect_matching (proposed)

/// Collect matching sweep entries plus the indexed table rows into `head`.
///
/// `this` is the pool and `head` points at the list head word. Returns at
/// once (passing the incoming register through, fixed by the proof) when
/// the enable byte at +0x73 is clear. Otherwise clears the state words,
/// then sweeps through the next callee (scripted to two entries then
/// done): each entry whose word at +0x28 selects bit 0x40 is resolved to
/// a node and pushed onto `head`. Then the table callee reports the row
/// count (0 or 2 in the proof): each row and sub-slot runs the cell
/// callee, and non-null cells resolve to nodes pushed the same way.
/// Returns whatever the final callee left in eax (scripted). The proof
/// pins the window and primes every branch.
///
/// Original: 0x00A8C440 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8c440(this: u32, head: u32) -> u32 {
    unsafe {
        const CALLEE_NEXT: u32 = 1;
        const CALLEE_RESOLVE: u32 = 2;
        const CALLEE_ROWS: u32 = 3;
        const CALLEE_CELL: u32 = 4;
        const RESOLVER: u32 = 0x12b4164;
        const ENABLE: u32 = 0x73;
        const STATE_A: u32 = 0xfc;
        const STATE_B: u32 = 0x100;
        const ENTRY_SEL: u32 = 0x28;
        const SELECT_BIT: u32 = 0x40;
        const SELECT_MASK: u32 = 0x3c0;
        const NODE_LINK: u32 = 4;
        const SUB_SLOTS: u32 = 4;
        const INCOMING_EAX: u32 = 0x12345678;
        let enabled =
            ((this + ENABLE) as *const u8).read_unaligned();
        if enabled == 0 {
            return INCOMING_EAX;
        }
        ((this + STATE_A) as *mut u32).write_unaligned(0);
        ((this + STATE_B) as *mut u32).write_unaligned(0xffffffff);
        let push_node = |head: u32, obj: u32| -> u32 {
            let resolver =
                lf_checker_rt::global::<u32>(RESOLVER).read_unaligned();
            let node =
                lf_checker_rt::callee_thiscall!(CALLEE_RESOLVE, u32, resolver);
            if node != 0 {
                (node as *mut u32).write_unaligned(obj);
            }
            let old = (head as *const u32).read_unaligned();
            ((node + NODE_LINK) as *mut u32).write_unaligned(old);
            (head as *mut u32).write_unaligned(node);
            node
        };
        let mut slot: u32 = 0;
        let slot_addr = core::ptr::addr_of_mut!(slot) as u32;
        let mut more = lf_checker_rt::callee_thiscall!(
            CALLEE_NEXT,
            u32,
            this,
            slot_addr
        );
        if more as u8 != 0 {
            loop {
                let entry = slot;
                if entry != 0 {
                    let sel = ((entry + ENTRY_SEL) as *const u32)
                        .read_unaligned();
                    if sel & SELECT_MASK == SELECT_BIT {
                        push_node(head, entry);
                    }
                }
                more = lf_checker_rt::callee_thiscall!(
                    CALLEE_NEXT,
                    u32,
                    this,
                    slot_addr
                );
                if more as u8 == 0 {
                    break;
                }
            }
        }
        let rows = lf_checker_rt::callee_thiscall!(
            CALLEE_ROWS,
            u32,
            this,
            0
        );
        // The original falls out with the last callee answer in eax.
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
                        last = push_node(head, cell);
                    }
                    sub += 1;
                }
                row += 1;
            }
        }
        last
    }
});
