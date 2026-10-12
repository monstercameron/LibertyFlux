// original: 0x00DE35B0 ui_refresh_clip_transition_state

/// Refresh the clip-transition state of the object in `this`. Its two child
/// pointers are stored at `+0x1e0` and `+0x1e4`; virtual methods first query
/// the second child, then update a stored time and pass the first child's
/// time as an x87 `f32` argument to a method on `this`. After that optional
/// update, the method always resets the first child and clears both children
/// through their virtual interfaces. The final child call's EAX value is
/// preserved as the function result. The gate is the low byte returned by
/// the second child's query method.
lf_checker_rt::export!(thiscall, rw_00DE35B0(this: u32) -> u32 {
    const FIRST_CHILD: u32 = 0x1e0;
    const SECOND_CHILD: u32 = 0x1e4;
    const STORED_TIME: u32 = 0x1e8;
    const TRANSITION_FLAG: u32 = 0x1ec;

    const CALLEE_QUERY_ACTIVE: u32 = 1;
    const CALLEE_READ_TIME: u32 = 2;
    const CALLEE_CHILD_TIME: u32 = 3;
    const CALLEE_SET_TIME: u32 = 4;
    const CALLEE_RESET_CHILD: u32 = 5;
    const CALLEE_CLEAR_FIRST: u32 = 6;
    const CALLEE_CLEAR_SECOND: u32 = 7;
    const CALLEE_FINISH_SECOND: u32 = 8;

    let second_child = unsafe {
        (this.wrapping_add(SECOND_CHILD) as *const u32).read_unaligned()
    };
    let active = lf_checker_rt::callee_thiscall!(CALLEE_QUERY_ACTIVE, u8, second_child);

    if active != 0 {
        let time_on_this = lf_checker_rt::callee_thiscall!(CALLEE_READ_TIME, f32, this);
        unsafe {
            (this.wrapping_add(STORED_TIME) as *mut f32).write_unaligned(time_on_this);
        }

        let first_child = unsafe {
            (this.wrapping_add(FIRST_CHILD) as *const u32).read_unaligned()
        };
        let child_time = lf_checker_rt::callee_thiscall!(CALLEE_CHILD_TIME, f32, first_child);
        let _ = lf_checker_rt::callee_thiscall!(CALLEE_SET_TIME, u32, this, child_time);
        unsafe {
            (this.wrapping_add(TRANSITION_FLAG) as *mut u8).write(0);
        }
    }

    let first_child = unsafe {
        (this.wrapping_add(FIRST_CHILD) as *const u32).read_unaligned()
    };
    let _ = lf_checker_rt::callee_thiscall!(CALLEE_RESET_CHILD, u32, first_child);
    let _ = lf_checker_rt::callee_thiscall!(CALLEE_CLEAR_FIRST, u32, first_child, 0u32);

    let second_child = unsafe {
        (this.wrapping_add(SECOND_CHILD) as *const u32).read_unaligned()
    };
    let _ = lf_checker_rt::callee_thiscall!(CALLEE_CLEAR_SECOND, u32, second_child, 0u32);
    lf_checker_rt::callee_thiscall!(CALLEE_FINISH_SECOND, u32, second_child, 0u32)
});
