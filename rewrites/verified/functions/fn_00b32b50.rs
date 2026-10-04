// original: 0x00b32b50 subtask_reset_and_count (proposed)

/// Reset a sub-task record, then tick a shared countdown byte down.
///
/// `target` points to the record (never null here); it is reset through a
/// callee first. Then the shared counter byte is read: when it is zero it is
/// left alone, otherwise it is decremented. Returns the callee's answer with
/// its low byte replaced by the counter's final value.
///
/// Original: 0x00b32b50 (cdecl, one stack word; one no-argument callee; the
/// null-target early return is not exercised because it returns the
/// unreadable incoming accumulator).
lf_checker_rt::export!(cdecl, rw_00b32b50(target: u32) -> u32 {
    unsafe {
        const RESET: u32 = 1;
        const COUNTER: u32 = 0x01661401;
        let answer: u32 = lf_checker_rt::callee_thiscall!(RESET, u32, target);
        let cell = lf_checker_rt::global::<u8>(COUNTER);
        let mut count = cell.read();
        if count != 0 {
            count = count.wrapping_sub(1);
            cell.write(count);
        }
        answer & !0xFF | count as u32
    }
});
