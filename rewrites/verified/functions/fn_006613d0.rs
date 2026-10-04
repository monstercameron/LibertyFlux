// original: 0x006613d0 rage::snLeaveGamersFromRlineTask::vf3
/// Drive the leave-gamers state machine one step.
///
/// Counts down the barrier at `+0x14` (a count reaching zero becomes -1).
/// In phase 1 (`+0x90`) with the leave unacknowledged (`+0x94` not 1),
/// completes through virtual slot 7, flagging whether the leave was
/// refused (`+0x94` is 3). Unless acknowledged, a final readiness check
/// runs, looping back while it reports ready. Returns the last answer.
export!(thiscall, rw_006613d0(this: u32, amount: u32) -> u32 {
    unsafe {
        let count = ((this + 0x14) as *const u32).read() as i32;
        if count > 0 {
            let rest = (count as u32).wrapping_sub(amount);
            ((this + 0x14) as *mut u32).write(if rest == 0 { 0xffff_ffff } else { rest });
        }
        loop {
            let phase = ((this + 0x90) as *const u32).read();
            let mut eax: u32;
            if phase == 1 {
                let ack = ((this + 0x94) as *const u32).read();
                if ack == 1 {
                    eax = 0;
                } else {
                    eax = task_complete(this, (ack == 3) as u32, 0);
                }
            } else {
                eax = phase.wrapping_sub(1);
            }
            if ((this + 0x94) as *const u32).read() == 1 {
                return eax;
            }
            let ready_now = callee_thiscall!(2, u32, this);
            if (ready_now as u8) == 0 {
                return ready_now;
            }
        }
    }
});

