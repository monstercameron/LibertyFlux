// original: 0x0064d2f0 rage::ptxSprite::vf24

/// Move the head node of one sprite pool to the front of another pool's
/// list. `this + 0x184` points at the source pool header; its head word is
/// unlinked (source count at `+0x1c` decremented, drain count at `+0x6c`
/// incremented) and pushed onto the front of `pool`'s list (target count at
/// `+0x1c` incremented, the node's link words set to the old head and null,
/// the old head's back-link updated when non-null). Returns the moved node,
/// or null when the source pool is empty. Thiscall, one stack argument.
lf_checker_rt::export!(thiscall, rw_0064d2f0(this: u32, pool: u32) -> u32 {
    unsafe {
        const POOL_HEADER: u32 = 0x184;
        const SRC_COUNT: u32 = 0x1c;
        const SRC_DRAINED: u32 = 0x6c;
        const DST_COUNT: u32 = 0x1c;
        const NODE_NEXT: u32 = 0;
        const NODE_PREV: u32 = 4;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let header = read_word(this.wrapping_add(POOL_HEADER));
        let node = read_word(header.wrapping_add(NODE_NEXT));
        if node == 0 {
            return 0;
        }
        let next = read_word(node.wrapping_add(NODE_NEXT));
        write_word(
            header.wrapping_add(SRC_COUNT),
            read_word(header.wrapping_add(SRC_COUNT)).wrapping_sub(1),
        );
        write_word(
            header.wrapping_add(SRC_DRAINED),
            read_word(header.wrapping_add(SRC_DRAINED)).wrapping_add(1),
        );
        write_word(header.wrapping_add(NODE_NEXT), next);
        let old_head = read_word(pool.wrapping_add(NODE_NEXT));
        if old_head != 0 {
            write_word(old_head.wrapping_add(NODE_PREV), node);
        }
        let head_now = read_word(pool.wrapping_add(NODE_NEXT));
        write_word(node.wrapping_add(NODE_NEXT), head_now);
        write_word(node.wrapping_add(NODE_PREV), 0);
        write_word(
            pool.wrapping_add(DST_COUNT),
            read_word(pool.wrapping_add(DST_COUNT)).wrapping_add(1),
        );
        write_word(pool.wrapping_add(NODE_NEXT), node);
        node
    }
});
