// original: 0x009e8940 task_chain_has_marker
/// Scans the task chain rooted at `[this+0x224]+0x2E0` for a node
/// whose word at `+4` is `0x51E`, returning 1 on a hit. The scan bails
/// out to the fallback when a later node outranks the first
/// (`(word8 >> 1) & 7` above the first node's, first at least 2). The
/// fallback, also used for an empty chain, is bit 1 of `+0x270`.
/// (thiscall; low byte of the return is the value.)
lf_checker_rt::export!(thiscall, rw_009e8940(this_ptr: u32) -> u32 {
    unsafe {
        const SLOTS_OFF: u32 = 0x224;
        const CHAIN_OFF: u32 = 0x2E0;
        const FLAG_OFF: u32 = 0x270;
        const MARKER: u32 = 0x51E;
        unsafe fn fallback(this_ptr: u32) -> u32 {
            (((this_ptr.wrapping_add(FLAG_OFF) as *const u32).read_unaligned() >> 1) & 1)
        }
        unsafe fn pri(node: u32) -> u32 {
            (((node.wrapping_add(8) as *const u32).read_unaligned() >> 1) & 7)
        }
        let slots = (this_ptr.wrapping_add(SLOTS_OFF) as *const u32).read_unaligned();
        let mut node = (slots.wrapping_add(CHAIN_OFF) as *const u32).read_unaligned();
        if node == 0 {
            return fallback(this_ptr);
        }
        let first = pri(node);
        loop {
            let cur = pri(node);
            if first < cur && first >= 2 {
                return fallback(this_ptr);
            }
            if (node.wrapping_add(4) as *const u32).read_unaligned() == MARKER {
                return 1;
            }
            node = (node.wrapping_add(12) as *const u32).read_unaligned();
            if node == 0 {
                return fallback(this_ptr);
            }
        }
    }
});
