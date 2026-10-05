// original: 0x00a8af40 pool_detach_head (proposed)

/// Detach the locked head node from this list, if the lock hands one over.
///
/// `this` is the list (head at +0, tail mark at +4, anchor at +8). Takes
/// the shared lock through the first callee, which delivers the live node
/// in its out word (modelled by the proof's stub writes); a null node
/// skips straight to unlock. Otherwise the node is unhooked from the head
/// (advancing the head past it) and from the tail mark (clearing it), then
/// handed to the detach callee with the anchor. Always releases through
/// the unlock callee and returns its answer, as the original leaves it in
/// eax. The lock slot addresses (frame pointers in the original) are
/// unobserved; only the pushed lock id, the out word and the stores are
/// compared.
///
/// Original: 0x00A8AF40 (thiscall, one unread stack word).
lf_checker_rt::export!(thiscall, rw_00a8af40(this: u32, _unused: u32) -> u32 {
    unsafe {
        const CALLEE_LOCK: u32 = 1;
        const CALLEE_DETACH: u32 = 2;
        const CALLEE_UNLOCK: u32 = 3;
        const LOCK_ID: u32 = 0x12fb1dc;
        const HEAD: u32 = 0;
        const TAIL: u32 = 4;
        const ANCHOR: u32 = 8;
        const NEXT: u32 = 4;
        let mut slot = [0u32; 3];
        let slot_addr = slot.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(
            CALLEE_LOCK,
            u32,
            slot_addr,
            lf_checker_rt::relocated(LOCK_ID)
        );
        let node = slot[2];
        if node != 0 {
            let head = ((this + HEAD) as *const u32).read_unaligned();
            if node == head {
                let next =
                    ((node + NEXT) as *const u32).read_unaligned();
                ((this + HEAD) as *mut u32).write_unaligned(next);
            }
            let tail = ((this + TAIL) as *const u32).read_unaligned();
            if node == tail {
                ((this + TAIL) as *mut u32).write_unaligned(0);
            }
            lf_checker_rt::callee_thiscall!(
                CALLEE_DETACH,
                u32,
                this.wrapping_add(ANCHOR),
                node
            );
        }
        lf_checker_rt::callee_thiscall!(CALLEE_UNLOCK, u32, slot_addr)
    }
});
