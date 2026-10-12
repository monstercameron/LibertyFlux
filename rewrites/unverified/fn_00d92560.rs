// original: 0x00D92560 audio_zone_recompute_max_entry_type

/// For a container with no primary entry, visits its four child pointers and
/// invokes the same thiscall helper for each non-null child. The checker
/// intercepts that direct self-call; the primary-entry table scan is not
/// exercised by this contract. `this` is preserved as the helper receiver.
lf_checker_rt::export!(thiscall, rw_00d92560(this: u32, container: u32) -> () {
    const CHILDREN_OFFSET: u32 = 0x30;
    const CHILD_COUNT: usize = 4;
    const CHILD_CALL: u32 = 1;

    let primary_entry = unsafe {
        (container.wrapping_add(0x2c) as *const u32).read_volatile()
    };
    if primary_entry == 0 {
        let children = container.wrapping_add(CHILDREN_OFFSET);
        let mut index = 0;
        while index < CHILD_COUNT {
            let child = unsafe {
                (children.wrapping_add(index as u32 * 4) as *const u32).read_volatile()
            };
            if child != 0 {
                let _ = lf_checker_rt::callee_thiscall!(CHILD_CALL, u32, this, child);
            }
            index += 1;
        }
    }
});
