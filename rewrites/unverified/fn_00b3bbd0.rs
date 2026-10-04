// original: 0x00b3bbd0 task_pending_remove (proposed)

/// Remove every occurrence of `value` from the pending-task id array: copy
/// the current ids aside, zero the count, then write back only the ids that
/// differ from `value` and store the surviving count. The count read is
/// trusted (the checked contract keeps it at most five, the frame copy's
/// capacity). Ends with the CRT security-cookie check, which preserves all
/// registers; the check's cookie argument derives from the stack address and
/// is therefore uncompared in the contract. Returns the original count (0
/// when it was not positive), matching the original's exit register.
/// Original: 0x00b3bbd0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00b3bbd0(value: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x016624ac;
        const IDS: u32 = 0x01662498;
        const COOKIE_CHECK: u32 = 1;
        const CAPACITY: usize = 6;
        let count = lf_checker_rt::global::<u32>(COUNT).read() as i32;
        if count <= 0 {
            lf_checker_rt::global::<u32>(COUNT).write(0);
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CHECK, u32,);
            return 0;
        }
        let ids = lf_checker_rt::global::<u32>(IDS);
        let mut kept = [0u32; CAPACITY];
        let mut n = 0usize;
        for i in 0..count as usize {
            let id = ids.add(i).read();
            if id != value {
                kept[n] = id;
                n += 1;
            }
        }
        for (i, id) in kept.iter().enumerate().take(n) {
            ids.add(i).write(*id);
        }
        lf_checker_rt::global::<u32>(COUNT).write(n as u32);
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CHECK, u32,);
        count as u32
    }
});
