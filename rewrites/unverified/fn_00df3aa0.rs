// original: 0x00DF3AA0 UITransitionViewer::vf94

/// Enable mouse input for this transition viewer.
///
/// The manager pointer is read from the relocated global slot. When its
/// state word at `+0x200` is zero, request mouse mode 1. If this viewer has a
/// list child, select row 12, activate its current item when present, and
/// set the child's active byte at `+0x0c`. The incoming stack word is unused
/// and is popped by the thiscall method. The original returns a void-style
/// result, so EAX is not part of this proof.
lf_checker_rt::export!(thiscall, rw_df3aa0(this: u32, _flags: u32) -> u32 {
    unsafe {
        const MANAGER_STATE: u32 = 0x200;
        const LIST_CHILD: u32 = 0x1BC;
        const LIST_CURRENT: u32 = 0x04;
        const LIST_ACTIVE: u32 = 0x0C;
        const CURRENT_VTABLE: u32 = 0x120;
        const MANAGER_GLOBAL: u32 = 0x18B6C8C;

        let manager = (lf_checker_rt::relocated(MANAGER_GLOBAL) as *const u32).read_unaligned();
        let manager_state = ((manager + MANAGER_STATE) as *const u32).read_unaligned();
        if manager_state == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, manager, 1);
            let child = ((this + LIST_CHILD) as *const u32).read_unaligned();
            if child != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, child, 12);
                let current = ((child + LIST_CURRENT) as *const u32).read_unaligned();
                if current != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, current, 1);
                }
                ((child + LIST_ACTIVE) as *mut u8).write(1);
            }
        }
        0
    }
});
