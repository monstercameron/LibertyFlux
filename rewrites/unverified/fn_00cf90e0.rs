// original: 0x00cf90e0 climb_task_pose_gate (proposed)

/// Gates a climb task's pose commit on the state word at `+0x14`: state 4
/// runs the height check (flagging `+0x78` when the adjusted height exceeds
/// 0.5) and stages the pose from `+0x60`, other states stage from `+0x40`.
/// A flagged task refreshes the controller at `+0xa80` and overrides the
/// staged height with adjusted height plus 1.0, while an unflagged one takes
/// the staged height from the block at `+0x20` of the target. The staged
/// triple is committed through the target's virtual slot 8 with arguments
/// (slot, 0, 1), the slot address skipped and its three words snapshotted.
/// Returns the commit's result.
///
/// Original: 0x00cf90e0 (thiscall: ecx holds the object, one stack word).
lf_checker_rt::export!(thiscall, rw_00cf90e0(this: u32, target: u32) -> u32 {
    unsafe {
        const UNIT_ADDR: u32 = 0x00fe88e8; // 1.0f
        const LIMIT_ADDR: u32 = 0x00fe8830; // 0.5f
        const REFRESH_CALLEE: u32 = 1;
        const COMMIT_SLOT: u32 = 8;
        ((this + 0x78) as *mut u8).write(0);
        let state = ((this + 0x14) as *const u32).read_unaligned();
        let base = if state == 4 {
            let unit = f32::from_bits((lf_checker_rt::global::<u32>(UNIT_ADDR)).read_unaligned());
            let limit =
                f32::from_bits((lf_checker_rt::global::<u32>(LIMIT_ADDR)).read_unaligned());
            let block = ((target + 0x20) as *const u32).read_unaligned();
            let h = f32::from_bits(((block + 0x38) as *const u32).read_unaligned());
            let cur = f32::from_bits(((this + 0x48) as *const u32).read_unaligned());
            let adj = core::hint::black_box(h) - core::hint::black_box(unit);
            let diff = core::hint::black_box(cur) - core::hint::black_box(adj);
            if diff > limit {
                ((this + 0x78) as *mut u8).write(1);
            }
            this.wrapping_add(0x60)
        } else {
            this.wrapping_add(0x40)
        };
        let mut slot = [0u32; 3];
        for i in 0..3u32 {
            slot[i as usize] = ((base + i * 4) as *const u32).read_unaligned();
        }
        let flag = ((this + 0x78) as *const u8).read();
        if flag != 0 {
            let unit = f32::from_bits((lf_checker_rt::global::<u32>(UNIT_ADDR)).read_unaligned());
            let cur = f32::from_bits(((this + 0x48) as *const u32).read_unaligned());
            let raised = core::hint::black_box(cur) + core::hint::black_box(unit);
            slot[2] = raised.to_bits();
            let ctl = ((target + 0xa80) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(REFRESH_CALLEE, u32, ctl, 1);
        } else {
            let block = ((target + 0x20) as *const u32).read_unaligned();
            slot[2] = ((block + 0x38) as *const u32).read_unaligned();
        }
        let vt = ((target + 0) as *const u32).read_unaligned();
        let commit: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(
            ((vt + COMMIT_SLOT) as *const u32).read_unaligned() as usize);
        commit(target, &slot as *const u32 as u32, 0, 1)
    }
});
