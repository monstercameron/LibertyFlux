// original: 0x00612230 physics_refresh_item_membership

/// Run the node refresh helper, detach `node` from the list embedded at
/// `this + 0x0c`, insert it before that list's first node, then decrement the
/// owner count at `this + 4` with wrapping arithmetic. All three helpers are
/// invoked through the checker runtime in the original call order. This is a
/// 32-bit thiscall with one node pointer on the stack.
lf_checker_rt::export!(thiscall, rw_00612230(this: u32, node: u32) -> () {
    unsafe {
        const REFRESH_NODE: u32 = 1;
        const DETACH_NODE: u32 = 2;
        const INSERT_FIRST: u32 = 3;
        let embedded_list = this.wrapping_add(0x0c);
        let _ = lf_checker_rt::callee_thiscall!(REFRESH_NODE, u32, node);
        let _ = lf_checker_rt::callee_thiscall!(DETACH_NODE, u32, embedded_list, node);
        let _ = lf_checker_rt::callee_thiscall!(INSERT_FIRST, u32, embedded_list, node);
        let count = ((this + 4) as *const u32).read_unaligned();
        ((this + 4) as *mut u32).write_unaligned(count.wrapping_sub(1));
    }
});
