// original: 0x00cc7010 queue_pop_front
/// Pop the head node off the intrusive queue at `this` and return it.
///
/// The successor becomes the head, the popped node's link is cleared, a tail
/// that pointed at the popped node is cleared, and the count at `+8`
/// decrements. Returns null when the queue is empty.
export!(thiscall, rw_00cc7010(this: *mut u8) -> u32 {
    unsafe {
        let head = *(this as *const u32);
        if head == 0 {
            return 0;
        }
        let next = *(((head) as *const u8).add(4) as *const u32);
        *(this as *mut u32) = next;
        *(((head) as *mut u8).add(4) as *mut u32) = 0;
        if *(this.add(4) as *const u32) == head {
            *(this.add(4) as *mut u32) = 0;
        }
        let count = *(this.add(8) as *const u32);
        *(this.add(8) as *mut u32) = count.wrapping_sub(1);
        head
    }
});
