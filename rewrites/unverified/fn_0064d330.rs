// original: 0x0064d330 rage::ptxSprite::vf9

/// Sweep the sprite list headed at `param`, refreshing every node whose
/// level at `+0xf0` reaches the global threshold: the node's block at
/// `+0xa0` is copied to the per-selector slot (selector byte from the table
/// the global at file VA `0x1bb6678` points at, slot `node + 0x100 + sel*96`)
/// through the block-copy callee, then the node is appended to the selector's
/// chain in `param` (count at `+0x14` bumped, head/tail words maintained,
/// the node's own link word cleared). Nodes below the threshold (or with a
/// not-a-number level, which also fails the ordered compare) keep their
/// place but are skipped. Stdcall, one stack argument, no return value.
lf_checker_rt::export!(stdcall, rw_0064d330(param: u32) -> u32 {
    unsafe {
        const THRESHOLD: u32 = 0x00fe_88e8;
        const SELECTOR_TABLE_PTR: u32 = 0x01bb_6678;
        const SELECTOR_BYTE: u32 = 0x01;
        const NODE_LEVEL: u32 = 0xf0;
        const NODE_BLOCK: u32 = 0xa0;
        const SLOT_BASE: u32 = 0x100;
        const SLOT_STRIDE: u32 = 96;
        const CHAIN_COUNT: u32 = 0x14;
        const CHAIN_HEAD: u32 = 0x04;
        const CHAIN_TAIL: u32 = 0x0c;
        const NODE_LINK: u32 = 0x08;
        const COPY_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let threshold = f32::from_bits(read_word(
            lf_checker_rt::relocated(THRESHOLD),
        ));
        let table = read_word(lf_checker_rt::relocated(SELECTOR_TABLE_PTR));
        let mut node = read_word(param);
        while node != 0 {
            let next = read_word(node);
            let level = f32::from_bits(read_word(node.wrapping_add(NODE_LEVEL)));
            if threshold >= level {
                let sel =
                    ((table.wrapping_add(SELECTOR_BYTE)) as *const u8).read() as u32;
                let slot = node
                    .wrapping_add(SLOT_BASE)
                    .wrapping_add(sel.wrapping_mul(SLOT_STRIDE));
                let block = node.wrapping_add(NODE_BLOCK);
                lf_checker_rt::callee_thiscall!(COPY_CALLEE, u32, slot, block);
                let lane = param.wrapping_add(sel.wrapping_mul(4));
                write_word(
                    lane.wrapping_add(CHAIN_COUNT),
                    read_word(lane.wrapping_add(CHAIN_COUNT)).wrapping_add(1),
                );
                if read_word(lane.wrapping_add(CHAIN_HEAD)) == 0 {
                    write_word(lane.wrapping_add(CHAIN_TAIL), 0);
                    write_word(lane.wrapping_add(CHAIN_HEAD), node);
                }
                let tail = read_word(lane.wrapping_add(CHAIN_TAIL));
                if tail != 0 {
                    write_word(tail.wrapping_add(sel.wrapping_mul(4)).wrapping_add(NODE_LINK), node);
                }
                write_word(
                    node.wrapping_add(sel.wrapping_mul(4)).wrapping_add(NODE_LINK),
                    0,
                );
                write_word(lane.wrapping_add(CHAIN_TAIL), node);
            }
            node = next;
        }
        0
    }
});
