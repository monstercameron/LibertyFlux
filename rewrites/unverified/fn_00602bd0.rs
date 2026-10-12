// original: 0x00602BD0 rage::rmPtfxManager::alloc_effect_node (proposed)

/// Pop one node off the manager's free list, link it at the head of the
/// active list, and return the node's body address (`node + 0x10`), after
/// clearing its state byte and recording the node address in it.
///
/// `this` points to the manager. The free-list head is the word at
/// `FREE_HEAD` (+0x190); the active-list tail is at `ACTIVE_TAIL`
/// (+0x210). Two counters gate the fast path: when the words at
/// `LIMIT` (+0x200) and `COUNT` (+0x1fc) are equal the pool is exhausted
/// and the function returns 0 without touching anything else. Otherwise
/// the head node is unlinked (`FREE_TOTAL` at +0x1ac decremented, `COUNT`
/// incremented), appended after the tail (an empty tail just becomes the
/// node), `ACTIVE_TOTAL` (+0x22c) is incremented, and the returned body
/// gets a zero state byte at `BODY_STATE` (+0x251) and the node address
/// at `BODY_NODE` (+0x260). A null free-list head also returns 0.
/// Thiscall: `this` in ECX, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00602BD0(this: u32) -> u32 {
    unsafe {
        const FREE_HEAD: u32 = 0x190;
        const FREE_TOTAL: u32 = 0x1ac;
        const COUNT: u32 = 0x1fc;
        const LIMIT: u32 = 0x200;
        const ACTIVE_TAIL: u32 = 0x210;
        const ACTIVE_TOTAL: u32 = 0x22c;
        const BODY: u32 = 0x10;
        const BODY_STATE: u32 = 0x251;
        const BODY_NODE: u32 = 0x260;

        #[inline(always)]
        unsafe fn rd(w: u32) -> u32 {
            unsafe { (w as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn wr(w: u32, v: u32) {
            unsafe { (w as *mut u32).write(v) }
        }

        if rd(this + LIMIT).wrapping_sub(rd(this + COUNT)) == 0 {
            return 0;
        }
        let node = rd(this + FREE_HEAD);
        if node == 0 {
            return 0;
        }
        let next = (node as *const u32).read();
        wr(this + FREE_TOTAL, rd(this + FREE_TOTAL).wrapping_sub(1));
        wr(this + COUNT, rd(this + COUNT).wrapping_add(1));
        wr(this + FREE_HEAD, next);
        let tail = rd(this + ACTIVE_TAIL);
        if tail != 0 {
            wr(tail + 4, node);
        }
        wr(node, rd(this + ACTIVE_TAIL));
        wr(node + 4, 0);
        wr(this + ACTIVE_TOTAL, rd(this + ACTIVE_TOTAL).wrapping_add(1));
        wr(this + ACTIVE_TAIL, node);
        let body = node.wrapping_add(BODY);
        ((body + BODY_STATE) as *mut u8).write(0);
        wr(body + BODY_NODE, node);
        body
    }
});
