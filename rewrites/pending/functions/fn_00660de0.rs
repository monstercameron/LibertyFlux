// original: 0x00660de0 rage::snJoinGamersToRlineTask::vf3
/// Drive the join-gamers state machine one step.
///
/// Counts down the barrier at `+0x14` (a count reaching zero becomes -1),
/// then acts on the phase at `+0x90`: phase 1 completes through virtual
/// slot 7 unless already acknowledged (`+0x94`); phase 0 scans the join
/// table for a ready slot and, when the session is handshaking
/// (states 2..=3), prepares the join and submits the gamer lists; any
/// other phase completes immediately. A submitted join advances the phase
/// to 1. Unless acknowledged, a final readiness check runs, looping back
/// while it reports ready. Returns the last callee answer or address.
export!(thiscall, rw_00660de0(this: u32, amount: u32) -> u32 {
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
            } else if phase != 0 {
                eax = phase.wrapping_sub(1);
            } else {
                let slots = ((this + 0x540) as *const u32).read() as i32;
                let mut ready = 0u32;
                let mut has_ready = false;
                if slots > 0 {
                    let mut entry = this + 0x420;
                    let mut i = 0i32;
                    loop {
                        if (entry as *const u32).read() == 1 {
                            ready = entry;
                            has_ready = true;
                            break;
                        }
                        i += 1;
                        entry += 8;
                        if !(i < slots) {
                            break;
                        }
                    }
                }
                if has_ready {
                    eax = ready;
                } else {
                    let session = ((this + 0x60) as *const u32).read();
                    let state = ((session + 0x50) as *const u32).read();
                    if state != 2 && state != 3 {
                        eax = task_complete(this, 0, 0);
                    } else {
                        callee_thiscall!(2, u32, this);
                        let members = ((this + 0x540) as *const u32).read();
                        if members == 0 {
                            eax = task_complete(this, 1, 0);
                        } else {
                            let submitted = callee_thiscall!(
                                3,
                                u32,
                                session + 0x48,
                                this + 0xa0,
                                this + 0x3a0,
                                members,
                                this + 0x94
                            );
                            if (submitted as u8) == 0 {
                                eax = task_complete(this, 0, 0);
                            } else {
                                ((this + 0x90) as *mut u32).write(1);
                                eax = submitted;
                            }
                        }
                    }
                }
            }
            if ((this + 0x94) as *const u32).read() == 1 {
                return eax;
            }
            let ready_now = callee_thiscall!(4, u32, this);
            if (ready_now as u8) == 0 {
                return ready_now;
            }
            if ((this + 0x90) as *const u32).read() == 0 {
                return ready_now;
            }
        }
    }
});

