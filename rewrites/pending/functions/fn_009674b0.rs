// original: 0x009674B0 release_owned_list
/// Clear `this`'s ownership flags and release its whole list.
///
/// First pass: every node of the list at `this + 0x30` whose flag word
/// (pointed to by the node's second word) holds `this` has that flag
/// cleared. Second pass: all nodes have their link word cleared, and the
/// whole chain is spliced onto the global free list at file VA
/// `0x01218474`, leaving `this + 0x30` empty. An empty list skips both
/// passes. The guard lock is callee 1; the function ends in a tail jump
/// to the unlock helper (callee 2), answered 0 as `eax` is 0 on every
/// path reaching it. Returns 0.
///
/// Original: 0x009674B0 (thiscall, no stack words; tail jump to unlock).

export!(thiscall, rw_009674B0(this: u32) -> u32 {
    unsafe {
        const FREE_HEAD: u32 = 0x01218474;
        const LIST: u32 = 0x30;
        callee_cdecl!(1, u32,);
        let mut node = (this.wrapping_add(LIST) as *const u32).read_unaligned();
        while node != 0 {
            let flag_ptr = (node.wrapping_add(4) as *const u32).read_unaligned();
            if (flag_ptr as *const u32).read_unaligned() == this {
                (flag_ptr as *mut u32).write_unaligned(0);
            }
            node = (node as *const u32).read_unaligned();
        }
        let head = (this.wrapping_add(LIST) as *const u32).read_unaligned();
        if head != 0 {
            let mut tail = head;
            (tail.wrapping_add(4) as *mut u32).write_unaligned(0);
            while (tail as *const u32).read_unaligned() != 0 {
                tail = (tail as *const u32).read_unaligned();
                (tail.wrapping_add(4) as *mut u32).write_unaligned(0);
            }
            let free = *global::<u32>(FREE_HEAD);
            (tail as *mut u32).write_unaligned(free);
            *global::<u32>(FREE_HEAD) = head;
            (this.wrapping_add(LIST) as *mut u32).write_unaligned(0);
        }
        callee_cdecl!(2, u32,);
        0
    }
});
