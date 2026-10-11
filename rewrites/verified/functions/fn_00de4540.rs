// original: 0x00DE4540 UIRowLayout::vf142

/// Call the child virtual method at `+0xFC` once for every child in the unsigned 16-bit count, then tail-forward the parent to its virtual method at `+0x1F4` and return that method's 32-bit result. The child method's return value is ignored.
lf_checker_rt::export!(thiscall, rw_00de4540(this_ptr: u32) -> u32 {
    const CHILD_LIST_OFFSET: u32 = 0x1D4;
    const CHILD_COUNT_OFFSET: u32 = 0x1D8;
    const CHILD_METHOD_SLOT: u32 = 0xFC;
    const PARENT_METHOD_SLOT: u32 = 0x1F4;
    let child_list = unsafe { ((this_ptr + CHILD_LIST_OFFSET) as *const u32).read_unaligned() };
    let child_count = unsafe { ((this_ptr + CHILD_COUNT_OFFSET) as *const u16).read_unaligned() } as u32;
    for index in 0..child_count {
        let child = unsafe { ((child_list + index * 4) as *const u32).read_unaligned() };
        let vtable = unsafe { (child as *const u32).read_unaligned() };
        let method = unsafe { ((vtable + CHILD_METHOD_SLOT) as *const u32).read_unaligned() };
        let visit: unsafe extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(method as usize) };
        let _ignored_child_result = unsafe { visit(child) };
    }
    let parent_vtable = unsafe { (this_ptr as *const u32).read_unaligned() };
    let parent_method = unsafe { ((parent_vtable + PARENT_METHOD_SLOT) as *const u32).read_unaligned() };
    let finish: unsafe extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(parent_method as usize) };
    unsafe { finish(this_ptr) }
});
