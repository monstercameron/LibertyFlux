// original: 0x00c5f920 is_active_driveby_sub_task

/// Checks the child task stored at parent offset `0x08`. A null child returns
/// false. Otherwise the method calls virtual slot `0x0C` on the child and
/// accepts only type `0x419`; it then returns true when the child's word at
/// offset `0x44` is either 3 or 4. The parent and child are 32-bit task
/// layouts, while the comparison and branch logic are pointer-width neutral.
lf_checker_rt::export!(thiscall, rw_00c5f920(parent: u32) -> u8 {
    const CHILD: u32 = 0x08;
    const TASK_TYPE: u32 = 0x419;
    const CHILD_STATE: u32 = 0x44;

    unsafe {
        let child = (parent.wrapping_add(CHILD) as *const u32).read_unaligned();
        if child == 0 {
            return 0;
        }
        let actual_type = lf_checker_rt::callee_thiscall!(1, u32, child);
        if actual_type != TASK_TYPE {
            return 0;
        }
        let state = (child.wrapping_add(CHILD_STATE) as *const u32).read_unaligned();
        u8::from(state == 3 || state == 4)
    }
});
