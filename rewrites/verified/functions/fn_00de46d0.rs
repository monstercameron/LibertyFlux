// original: 0x00DE46D0 UIRowLayout::vf72

/// Forward the supplied 32-bit word to each child's virtual method at `+0x120`, in child-list order, for the unsigned 16-bit child count. The method returns no meaningful value; the child calls use `thiscall` and consume one stack word each.
lf_checker_rt::export!(thiscall, rw_00de46d0(this_ptr: u32, forwarded_word: u32) -> u32 {
    const CHILD_LIST_OFFSET: u32 = 0x1D4;
    const CHILD_COUNT_OFFSET: u32 = 0x1D8;
    const CHILD_METHOD_SLOT: u32 = 0x120;
    let child_list = unsafe { ((this_ptr + CHILD_LIST_OFFSET) as *const u32).read_unaligned() };
    let child_count = unsafe { ((this_ptr + CHILD_COUNT_OFFSET) as *const u16).read_unaligned() } as u32;
    for index in 0..child_count {
        let child = unsafe { ((child_list + index * 4) as *const u32).read_unaligned() };
        let vtable = unsafe { (child as *const u32).read_unaligned() };
        let method = unsafe { ((vtable + CHILD_METHOD_SLOT) as *const u32).read_unaligned() };
        let notify: unsafe extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(method as usize) };
        let _ignored_child_result = unsafe { notify(child, forwarded_word) };
    }
    0
});
