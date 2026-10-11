// original: 0x005E8D60 CTaskComplexWanderCop::poll_chase_events

/// Advance the chase timer and select a target once its expiry time is reached.
///
/// `this` contains the timer start at `+0x84`, duration at `+0x88`, active
/// flag at `+0x8C`, reset flag at `+0x8D`, and expiry time at `+0x9C`. The
/// global millisecond clock initializes or resets the timer, and the deadline
/// check is signed (`start.wrapping_add(duration) <= now`). The final expiry
/// test is unsigned (`now >= expiry`); when it succeeds, the function calls
/// the target selector with the event pointer. The selector's return is
/// ignored and this method always returns zero after popping one stack word.
lf_checker_rt::export!(thiscall, rw_005e8d60(this: u32, event: u32) -> u32 {
    const TIMER_START_OFFSET: u32 = 0x84;
    const TIMER_DURATION_OFFSET: u32 = 0x88;
    const TIMER_ACTIVE_OFFSET: u32 = 0x8c;
    const TIMER_RESET_OFFSET: u32 = 0x8d;
    const EXPIRY_TIME_OFFSET: u32 = 0x9c;
    const TIMER_DURATION: u32 = 0x32;
    const MILLISECOND_CLOCK_VA: u32 = 0x011735B4;

    let active = unsafe {
        (this.wrapping_add(TIMER_ACTIVE_OFFSET) as *const u8).read()
    };
    if active == 0 {
        let initialization_time = unsafe {
            lf_checker_rt::global::<u32>(MILLISECOND_CLOCK_VA).read_unaligned()
        };
        unsafe {
            (this.wrapping_add(TIMER_START_OFFSET) as *mut u32)
                .write_unaligned(initialization_time);
            (this.wrapping_add(TIMER_DURATION_OFFSET) as *mut u32)
                .write_unaligned(TIMER_DURATION);
            (this.wrapping_add(TIMER_ACTIVE_OFFSET) as *mut u8).write(1);
        }
    }

    let active = unsafe {
        (this.wrapping_add(TIMER_ACTIVE_OFFSET) as *const u8).read()
    };
    if active == 0 {
        return 0;
    }
    let reset = unsafe {
        (this.wrapping_add(TIMER_RESET_OFFSET) as *const u8).read()
    };
    if reset != 0 {
        let reset_time = unsafe {
            lf_checker_rt::global::<u32>(MILLISECOND_CLOCK_VA).read_unaligned()
        };
        unsafe {
            (this.wrapping_add(TIMER_START_OFFSET) as *mut u32).write_unaligned(reset_time);
            (this.wrapping_add(TIMER_RESET_OFFSET) as *mut u8).write(0);
        }
    }

    let timer_start = unsafe {
        (this.wrapping_add(TIMER_START_OFFSET) as *const u32).read_unaligned()
    };
    let timer_duration = unsafe {
        (this.wrapping_add(TIMER_DURATION_OFFSET) as *const u32).read_unaligned()
    };
    let deadline = timer_start.wrapping_add(timer_duration);
    let deadline_check_time = unsafe {
        lf_checker_rt::global::<u32>(MILLISECOND_CLOCK_VA).read_unaligned()
    };
    if (deadline as i32) > (deadline_check_time as i32) {
        return 0;
    }
    unsafe {
        (this.wrapping_add(TIMER_START_OFFSET) as *mut u32)
            .write_unaligned(deadline_check_time);
        (this.wrapping_add(TIMER_DURATION_OFFSET) as *mut u32)
            .write_unaligned(TIMER_DURATION);
        (this.wrapping_add(TIMER_ACTIVE_OFFSET) as *mut u8).write(1);
    }

    let expiry = unsafe {
        (this.wrapping_add(EXPIRY_TIME_OFFSET) as *const u32).read_unaligned()
    };
    let expiry_check_time = unsafe {
        lf_checker_rt::global::<u32>(MILLISECOND_CLOCK_VA).read_unaligned()
    };
    if expiry_check_time >= expiry {
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, event);
    }
    0
});
