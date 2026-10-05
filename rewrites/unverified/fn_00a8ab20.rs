// original: 0x00a8ab20 pool_drain_queues (proposed)

/// Drain whichever queue still holds work, reporting whether any did.
///
/// `mode` selects the two-shot (non-zero low byte) or three-shot
/// shape; `this` supplies the worker id's upper bytes.
/// First the shared toggle byte is flipped to whether it was zero: when
/// set, queue A is asked for its depth and, if above the shape's limit,
/// runs one worker. Then queue B is asked and, if above 2, runs its own
/// worker. Then queue A is asked again against the same limit. Returns 1
/// when any worker ran and answered true, else 0 (the scripts only answer
/// 0/1 with small depths, so the full word is deterministic).
///
/// Original: 0x00A8AB20 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8ab20(this: u32, mode: u32) -> u32 {
    unsafe {
        const CALLEE_DEPTH: u32 = 1;
        const CALLEE_WORK_A: u32 = 2;
        const CALLEE_WORK_B: u32 = 3;
        const QUEUE_A: u32 = 0x16dceb8;
        const QUEUE_B: u32 = 0x16dd2bc;
        const TOGGLE: u32 = 0x12fb200;
        const B_LIMIT: i32 = 2;
        // Both queue words are relocated immediates in the original.
        let queue_a = lf_checker_rt::relocated(QUEUE_A);
        let queue_b = lf_checker_rt::relocated(QUEUE_B);
        let two_shot = mode as u8 != 0;
        let limit: i32 = if two_shot { 2 } else { 3 };
        // The worker id is the entry ecx slot (this) with its low byte
        // replaced by the shape flag, not the mode word.
        let worker =
            (this & 0xffffff00) | (if two_shot { 1 } else { 0 });
        let was_zero =
            (lf_checker_rt::global::<u8>(TOGGLE).read_unaligned() == 0)
                as u32;
        lf_checker_rt::global::<u8>(TOGGLE).write_unaligned(was_zero as u8);
        if was_zero != 0 {
            let depth_a = lf_checker_rt::callee_thiscall!(
                CALLEE_DEPTH,
                u32,
                queue_a
            ) as i32;
            if depth_a > limit {
                let done = lf_checker_rt::callee_thiscall!(
                    CALLEE_WORK_A,
                    u32,
                    queue_a,
                    worker
                );
                if done != 0 {
                    return 1;
                }
            }
        }
        let depth_b = lf_checker_rt::callee_thiscall!(
            CALLEE_DEPTH,
            u32,
            queue_b
        ) as i32;
        if depth_b > B_LIMIT {
            let done = lf_checker_rt::callee_thiscall!(
                CALLEE_WORK_B,
                u32,
                queue_a,
                worker
            );
            if done != 0 {
                return 1;
            }
        }
        let depth_a2 = lf_checker_rt::callee_thiscall!(
            CALLEE_DEPTH,
            u32,
            queue_a
        ) as i32;
        if depth_a2 > limit {
            let done = lf_checker_rt::callee_thiscall!(
                CALLEE_WORK_A,
                u32,
                queue_a,
                worker
            );
            if done != 0 {
                return 1;
            }
        }
        0
    }
});
