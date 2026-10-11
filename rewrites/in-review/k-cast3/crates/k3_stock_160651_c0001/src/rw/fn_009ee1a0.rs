// original: 0x009ee1a0 CTaskSimpleMoveDoNothing::vf17
/// Do-nothing move task tick: count down, then run the mover when live.
///
/// When the counter at `0x20` is positive it is decremented by the
/// product of two shared constants truncated toward zero (x87 chop),
/// and a counter at or below zero afterwards finishes the task with 1
/// (full `eax` keeps the chopped value's upper bytes with low byte 1).
/// Otherwise the subject argument is examined: a subject whose `0x29c`
/// word has bit `0x400` set ends the tick with 0, else the subject's
/// mover at `[0xa80]` slot `0x50` runs with 0 and the tick ends with 0
/// (low byte cleared, upper bytes whatever the path carried).
export!(thiscall, rw_009ee1a0(this_ptr: u32, subject: u32) -> u32 {
    unsafe {
        let counter = *((this_ptr + 0x20) as *const u32);
        if (counter as i32) > 0 {
            let a = f32::from_bits(*global::<u32>(0x11735bc));
            let b = f32::from_bits(*global::<u32>(0xfe8c58));
            let chopped = (a * b) as i64 as u32;
            let next = counter.wrapping_sub(chopped);
            *((this_ptr + 0x20) as *mut u32) = next;
            if (next as i32) <= 0 {
                return (chopped & 0xffffff00) | 1;
            }
            return task_tick_finish(subject, chopped);
        }
        task_tick_finish(subject, 0)
    }
});

/// Shared tail of the do-nothing move tick: gate on the subject's `0x400`
/// bit, else run its mover. Placed outside the export so both paths share it.
#[inline(always)]
unsafe fn task_tick_finish(subject: u32, carried: u32) -> u32 {
    unsafe {
        if *((subject + 0x29c) as *const u32) & 0x400 != 0 {
            return carried & 0xffffff00;
        }
        let mover = *((subject + 0xa80) as *const u32);
        let vt = *(mover as *const u32);
        let tgt = *((vt + 0x50) as *const u32);
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let ans = f(mover, 0);
        ans & 0xffffff00
    }
}
