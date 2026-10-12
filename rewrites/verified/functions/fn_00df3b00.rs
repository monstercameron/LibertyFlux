// original: 0x00DF3B00 UITransitionViewer::vf112

/// Bind the controller frame for this transition viewer.
///
/// Resolve the controller frame from the interned layout names, invoke the
/// frame's setup slot, and ask this viewer whether its alternate binding is
/// active. When that predicate is false, resolve the alternate frame, bind
/// it, and notify the viewer's two update slots. The final tail call forwards
/// `this` and supplies the method's return value.
lf_checker_rt::export!(thiscall, rw_df3b00(this: u32) -> u32 {
    unsafe {
        const FRAME_SETUP_SLOT: u32 = 0x1B0;
        const IS_BOUND_SLOT: u32 = 0x1C;
        const BIND_ALT_SLOT: u32 = 0x1AC;
        const ENABLE_SLOT: u32 = 0x18;
        const CONTROLLER_TREE: u32 = 0x1981A4C;
        const PRIMARY_NAME: u32 = 0xF017CC;
        const ALTERNATE_NAME: u32 = 0xF017E0;

        let primary = lf_checker_rt::callee_thiscall!(
            1, u32, lf_checker_rt::relocated(CONTROLLER_TREE),
            lf_checker_rt::relocated(PRIMARY_NAME)
        );
        let primary_vtable = (primary as *const u32).read_unaligned();
        let _primary_setup = ((primary_vtable + FRAME_SETUP_SLOT) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, primary);
        let viewer_vtable = (this as *const u32).read_unaligned();
        let _is_bound_target = ((viewer_vtable + IS_BOUND_SLOT) as *const u32).read_unaligned();
        let is_bound = lf_checker_rt::callee_thiscall!(3, u32, this);
        if (is_bound as u8) == 0 {
            let alternate = lf_checker_rt::callee_thiscall!(
                1, u32, lf_checker_rt::relocated(CONTROLLER_TREE),
                lf_checker_rt::relocated(ALTERNATE_NAME)
            );
            let alternate_vtable = (alternate as *const u32).read_unaligned();
            let _alternate_setup = ((alternate_vtable + FRAME_SETUP_SLOT) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, alternate);
            let vtable_for_bind = (this as *const u32).read_unaligned();
            let _bind_target = ((vtable_for_bind + BIND_ALT_SLOT) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, this);
            let vtable_for_enable = (this as *const u32).read_unaligned();
            let _enable_target = ((vtable_for_enable + ENABLE_SLOT) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, this, 1);
        }
        lf_checker_rt::callee_thiscall!(6, u32, this)
    }
});
