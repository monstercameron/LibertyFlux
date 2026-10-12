// original: 0x00DF3C40 UIFileViewer::apply_state_to_child_objects

/// Forward one state word to each present list child in the four child slots.
///
/// Null child pointers are skipped. The original is void-style and its EAX
/// value is not a defined return channel.
lf_checker_rt::export!(thiscall, rw_df3c40(this: u32, state: u32) -> u32 {
    unsafe {
        const CHILD_SLOTS: [u32; 4] = [0x1E0, 0x1E4, 0x1E8, 0x1EC];
        for field_offset in CHILD_SLOTS {
            let child = ((this + field_offset) as *const u32).read_unaligned();
            if child != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, child, state);
            }
        }
        0
    }
});
