// original: 0x00B6CD00 heli_rebind_slot_node

/// Replace the linked node pointer at `this + 0x2050`. If the pointer changes,
/// detach the old nonnull node, store the new value, and attach the new
/// nonnull node; both helpers receive the address of the slot. The routine is
/// void, so its unspecified EAX value is not part of the contract.
lf_checker_rt::export!(thiscall, rw_00B6CD00(this: u32, new_node: u32) -> () {
    unsafe {
        const NODE_SLOT: u32 = 0x2050;
        const DETACH_NODE: u32 = 1;
        const ATTACH_NODE: u32 = 2;
        let slot = this.wrapping_add(NODE_SLOT);
        let old_node = (slot as *const u32).read_unaligned();
        if old_node == new_node {
            return;
        }
        if old_node != 0 {
            let _ = lf_checker_rt::callee_thiscall!(DETACH_NODE, u32, old_node, slot);
        }
        (slot as *mut u32).write_unaligned(new_node);
        if new_node != 0 {
            let _ = lf_checker_rt::callee_thiscall!(ATTACH_NODE, u32, new_node, slot);
        }
    }
});
