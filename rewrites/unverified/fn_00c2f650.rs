// original: 0x00C2F650 detach_owned_timing_node

/// Inspect the embedded timing-node pointer at byte offset `0x24`. If it is
/// non-null, call the detachment routine with that node as the thiscall object
/// and the address of the owning object's pointer field as its sole stack
/// argument. The node pointer and owner are not modified directly by this
/// wrapper. A null pointer returns without making a call.
lf_checker_rt::export!(thiscall, rw_00c2f650(this: *mut u8) -> () {
    const TIMING_NODE_OFFSET: usize = 0x24;
    const DETACH_TIMING_NODE: u32 = 1;

    unsafe {
        let node = this.add(TIMING_NODE_OFFSET).cast::<u32>().read_unaligned();
        if node != 0 {
            let owner_field = this.add(TIMING_NODE_OFFSET) as u32;
            let _: u32 = lf_checker_rt::callee_thiscall!(
                DETACH_TIMING_NODE,
                u32,
                node,
                owner_field,
            );
        }
    }
});
