// original: 0x006f6540 queue_filter_drain
/// Detaches every unflagged node of the slot-0x50 queue and releases it.
///
/// Walks the queue, skipping nodes whose header has bit 5 of byte 1 set. Each
/// other node is unlinked through the shared pop helper, then its header and
/// body are handed to the owner's release handler in turn.
export!(thiscall, rw_006f6540(this: u32) -> () {
    unsafe {
        let head = this.wrapping_add(0x50);
        let mut prev_next = head;
        let mut cur = *(head as *const u32);
        if cur == 0 {
            return;
        }
        let mut next = *((cur + 4) as *const u32);
        loop {
            let header = *(cur as *const u32);
            let flag = (*((header + 1) as *const u8) << 2) >> 7;
            if flag & 1 == 0 {
                callee_thiscall!(1, u32, head, cur);
                let owner = *((this + 0x1c) as *const u32);
                let table = *(owner as *const u32);
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(*(table as *const u32).add(3));
                release(owner, header);
                let owner = *((this + 0x1c) as *const u32);
                let table = *(owner as *const u32);
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(*(table as *const u32).add(3));
                release(owner, cur);
            }
            cur = next;
            if next != 0 {
                next = *((next + 4) as *const u32);
            } else if prev_next & 0xFFFF_FFFE != 0 && prev_next & 1 != 0 {
                prev_next &= 0xFFFF_FFFE;
                next = *(prev_next as *const u32);
            }
            if cur == 0 {
                break;
            }
        }
    }
});
