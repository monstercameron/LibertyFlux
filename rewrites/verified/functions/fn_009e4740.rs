// original: 0x009e4740 angle_check_routed (proposed)

/// Route an angle check through the linked leaf or the inline block.
///
/// When `[p + 0x20]` is null the check runs on `p + 0x10`, otherwise on
/// `[p + 0x20] + 0x30`, always forwarding this object, the chosen pointer
/// and the float limit to the angle callee (`thiscall`, two stack words:
/// pointer, float) and returning its result.
lf_checker_rt::export!(thiscall, rw_009e4740(this: u32, p: u32, f: u32) -> u32 {
    unsafe {
        const LINK_OFF: u32 = 0x20;
        const ALT_OFF: u32 = 0x10;
        const LEAF_OFF: u32 = 0x30;
        const ANGLE_CALLEE: u32 = 1;
        let link = ((p + LINK_OFF) as *const u32).read_unaligned();
        let q = if link == 0 {
            p.wrapping_add(ALT_OFF)
        } else {
            link.wrapping_add(LEAF_OFF)
        };
        lf_checker_rt::callee_thiscall!(ANGLE_CALLEE, u32, this, q, f)
    }
});
