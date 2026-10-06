// original: 0x00872800 child_find_and_detach
/// Find a child in the list and detach it.
///
/// The child list is headed at `this + 0x14`, linked through `+0x0C`.
/// Returns 0 unless the candidate's back-link at `+8` points at `this`
/// and the candidate is found in the list. On a match, the candidate is
/// notified through virtual slot `0x0C` (thiscall with a two-word
/// `{0x10002, 0}` request built on the stack), unlinked from the list,
/// has its `+8`/`+0x0C` words cleared, and is released through virtual
/// slot 8. Returns 1 on a detach, else 0 (low byte meaningful).
///
/// Original: 0x00872800 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00872800(this: u32, child: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x14;
        const NEXT_OFF: u32 = 0x0C;
        const PARENT_OFF: u32 = 8;
        const NOTIFY_SLOT: u32 = 0x0C;
        const RELEASE_SLOT: u32 = 8;
        const REQ_KIND: u32 = 0x10002;
        if ((child + PARENT_OFF) as *const u32).read_unaligned() != this {
            return 0;
        }
        let mut link = this + HEAD_OFF;
        let mut node = (link as *const u32).read_unaligned();
        while node != 0 {
            if node == child {
                let request = [REQ_KIND, 0u32];
                let vtable = (node as *const u32).read_unaligned();
                let target = ((vtable + NOTIFY_SLOT) as *const u32).read_unaligned();
                let notify: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                notify(node, request.as_ptr() as u32);
                let next = ((node + NEXT_OFF) as *const u32).read_unaligned();
                (link as *mut u32).write_unaligned(next);
                let vtable2 = (node as *const u32).read_unaligned();
                ((node + NEXT_OFF) as *mut u32).write_unaligned(0);
                ((node + PARENT_OFF) as *mut u32).write_unaligned(0);
                let target2 = ((vtable2 + RELEASE_SLOT) as *const u32).read_unaligned();
                let release: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(target2 as usize);
                release(node);
                return 1;
            }
            link = node + NEXT_OFF;
            node = (link as *const u32).read_unaligned();
        }
        0
    }
});
