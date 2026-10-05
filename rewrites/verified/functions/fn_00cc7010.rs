// original: 0x00CC7010 list_pop_front (proposed)

/// Unlink and return the head node of an intrusive queue.
///
/// `this` points to a 12-byte header: word `+0` is the head node, `+4` the
/// tail node, `+8` the entry count. Each node links through its word `+4`. A
/// null head returns null with nothing touched. Otherwise the head's
/// successor becomes the head, the removed node's link is cleared, the tail
/// is cleared when it was the removed node, the count is decremented
/// (wrapping), and the removed node is returned.
///
/// Original: 0x00CC7010 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00cc7010(this: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0;
        const TAIL: u32 = 4;
        const COUNT: u32 = 8;
        const NEXT: u32 = 4;
        let head = (this.wrapping_add(HEAD) as *const u32).read_unaligned();
        if head == 0 {
            return head;
        }
        let next = (head.wrapping_add(NEXT) as *const u32).read_unaligned();
        (this.wrapping_add(HEAD) as *mut u32).write_unaligned(next);
        (head.wrapping_add(NEXT) as *mut u32).write_unaligned(0);
        if (this.wrapping_add(TAIL) as *const u32).read_unaligned() == head {
            (this.wrapping_add(TAIL) as *mut u32).write_unaligned(0);
        }
        let count = (this.wrapping_add(COUNT) as *const u32).read_unaligned();
        (this.wrapping_add(COUNT) as *mut u32).write_unaligned(count.wrapping_sub(1));
        head
    }
});
