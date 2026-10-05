// original: 0x00be7870 CTaskSimpleTired::vf17

/// Report whether a tired-task's rest timer has expired, latching on success.
///
/// `this` points to the task, `ped` to the ped. Starts the underlying action
/// through callee 1 (thiscall with `this`, one word: the ped) when the
/// started field at `+0x18` is zero. Returns 0 while the armed flag at
/// `+0x24` is clear. When the re-stamp flag at `+0x25` is set, reloads the
/// start stamp at `+0x1c` from the shared clock (dword at file address
/// `0x11735B4`) and clears the flag. Returns 0 while the signed sum of the
/// duration at `+0x20` and the stamp still exceeds the clock. Otherwise, when
/// bit 0 of the state at `+0x0c` is clear, polls the task's virtual slot
/// 0x14 (callee 2, thiscall, three words: ped, 0, 0) and sets bit 1 of the
/// state when the poll answers non-zero. Returns 1 once the timer expires.
///
/// Original: thiscall, one stack word, the callee pops 4 bytes, returns `al`.
lf_checker_rt::export!(thiscall, rw_00be7870(this: u32, ped: u32) -> u32 {
    unsafe {
        const OFF_STATE: u32 = 0x0c;
        const OFF_STAMP: u32 = 0x1c;
        const OFF_STARTED: u32 = 0x18;
        const OFF_DURATION: u32 = 0x20;
        const OFF_ARMED: u32 = 0x24;
        const OFF_RESTAMP: u32 = 0x25;
        const CLOCK_GLOBAL: u32 = 0x11735B4;
        const POLL_SLOT: u32 = 0x14;
        const SKIP_POLL_BIT: u32 = 1;
        const POLLED_BIT: u32 = 2;
        const STARTER: u32 = 1;
        const POLL: u32 = 2;

        if ((this + OFF_STARTED) as *const u32).read_unaligned() == 0 {
            lf_checker_rt::callee_thiscall!(STARTER, u32, this, ped);
        }
        if ((this + OFF_ARMED) as *const u8).read() == 0 {
            return 0;
        }
        if ((this + OFF_RESTAMP) as *const u8).read() != 0 {
            let now = lf_checker_rt::global::<u32>(CLOCK_GLOBAL).read_unaligned();
            ((this + OFF_STAMP) as *mut u32).write_unaligned(now);
            ((this + OFF_RESTAMP) as *mut u8).write(0);
        }
        let now = lf_checker_rt::global::<u32>(CLOCK_GLOBAL).read_unaligned();
        let stamp = ((this + OFF_STAMP) as *const u32).read_unaligned();
        let duration = ((this + OFF_DURATION) as *const u32).read_unaligned();
        if (duration.wrapping_add(stamp) as i32) > (now as i32) {
            return 0;
        }
        let state = ((this + OFF_STATE) as *const u32).read_unaligned();
        if state & SKIP_POLL_BIT == 0 {
            let vtable = (this as *const u32).read_unaligned();
            let slot = ((vtable + POLL_SLOT) as *const u32).read_unaligned();
            let poll: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            if poll(this, ped, 0, 0) as u8 != 0 {
                ((this + OFF_STATE) as *mut u32).write_unaligned(state | POLLED_BIT);
            }
        }
        1
    }
});
