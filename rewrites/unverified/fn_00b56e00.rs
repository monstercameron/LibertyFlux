// original: 0x00B56E00 list_pop_front_8c_90

/// Remove the list's current tail node, using link offset `0x90` for the new tail. If the new tail is null, clear the head too. Call the matching unlink routine only for a non-null node, and return the removed node pointer or zero.
lf_checker_rt::export!(thiscall, rw_bb56e00(this: u32) -> u32 {
    unsafe {
        const LIST_HEAD: usize = 0;
        const LIST_TAIL: usize = 4;
        const NODE_NEXT: usize = 0x90;
        let list_ptr = this as *mut u8;
        let node = (list_ptr.add(LIST_TAIL) as *const u32).read_unaligned();
        if node != 0 {
            let new_tail = ((node as *const u8).add(NODE_NEXT) as *const u32).read_unaligned();
            (list_ptr.add(LIST_TAIL) as *mut u32).write_unaligned(new_tail);
            if new_tail == 0 { (list_ptr.add(LIST_HEAD) as *mut u32).write_unaligned(0); }
            let _ = lf_checker_rt::callee_thiscall!(1, u32, node);
        }
        node
    }
});
