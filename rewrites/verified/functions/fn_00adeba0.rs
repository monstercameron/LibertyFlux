// original: 0x00adeba0 ui_splice_head_and_forward5
/// Splice `*head` in front of a dword run, then describe the run.
///
/// Writes `*head` over the word before `tail`, counts the dwords from `head`
/// to `tail` (exclusive of the spliced word), and calls the worker with
/// (head, 0, count, previous word, extra). Returns the worker's answer.
export!(cdecl, rw_00adeba0(head: *const u32, tail: *const u32, extra: u32) -> u32 {
    unsafe {
        let head_addr = head as u32;
        let tail_addr = tail as u32;
        let prev_slot = (tail_addr.wrapping_sub(4)) as *mut u32;
        let prev = *prev_slot;
        *prev_slot = *head;
        let count = (tail_addr.wrapping_sub(head_addr).wrapping_sub(4) as i32 >> 2) as u32;
        callee_cdecl!(1, u32, head_addr, 0, count, prev, extra)
    }
});
