// original: 0x00e706f0 drain_queue_1a0b0ec
/// Drain the queue rooted at 0x01A0B0EC (head) with its count at +8.
///
/// While the count word is nonzero, passes the head pointer to the dequeue
/// callee (thiscall/1 on the queue root) and re-reads both words. Returns
/// the last dequeue answer. Like the original, it spins without calling
/// when the count is nonzero while the head is null; contracts exclude that
/// input because the original never returns on it either.
export!(cdecl, rw_00e706f0() -> u32 {
    unsafe {
        const QUEUE: u32 = 0x01A0B0EC;
        let q = relocated(QUEUE);
        let mut last = 0u32;
        loop {
            let count = core::ptr::read_unaligned((q + 8) as *const u32);
            if count == 0 {
                break;
            }
            let head = core::ptr::read_unaligned(q as *const u32);
            if head != 0 {
                last = lf_checker_rt::callee_thiscall!(1, u32, q, head);
            }
        }
        last
    }
});
