// original: 0x00872880 child_list_teardown
/// Tear down the whole child list.
///
/// Walks the list headed at `this + 0x14` (linked through `+0x0C`); each
/// node is notified through virtual slot `0x0C` (thiscall with a two-word
/// `{0x10002, 0}` request built on the stack), has its `+8`/`+0x0C` words
/// cleared, and is released through virtual slot 8. The head is then
/// nulled. No return value.
///
/// Original: 0x00872880 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00872880(this: u32) -> u32 {
    unsafe {
        const HEAD_OFF: u32 = 0x14;
        const NEXT_OFF: u32 = 0x0C;
        const PARENT_OFF: u32 = 8;
        const NOTIFY_SLOT: u32 = 0x0C;
        const RELEASE_SLOT: u32 = 8;
        const REQ_KIND: u32 = 0x10002;
        let mut node = ((this + HEAD_OFF) as *const u32).read_unaligned();
        while node != 0 {
            let vtable = (node as *const u32).read_unaligned();
            let next = ((node + NEXT_OFF) as *const u32).read_unaligned();
            let request = [REQ_KIND, 0u32];
            let target = ((vtable + NOTIFY_SLOT) as *const u32).read_unaligned();
            let notify: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            notify(node, request.as_ptr() as u32);
            let vtable2 = (node as *const u32).read_unaligned();
            ((node + NEXT_OFF) as *mut u32).write_unaligned(0);
            ((node + PARENT_OFF) as *mut u32).write_unaligned(0);
            let target2 = ((vtable2 + RELEASE_SLOT) as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target2 as usize);
            release(node);
            node = next;
        }
        ((this + HEAD_OFF) as *mut u32).write_unaligned(0);
    }
    0
});
