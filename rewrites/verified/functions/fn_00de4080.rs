// original: 0x00DE4080 UIRowLayout::find_child_by_key

/// Return the first zero-based child index whose virtual key equals the supplied word, or `u32::MAX` when there is no match. The child count is an unsigned 16-bit field; the key comparison is exact 32-bit equality. Each lookup calls the child's virtual method at `+0x4C` using `thiscall`.
lf_checker_rt::export!(thiscall, rw_00de4080(this_ptr: u32, key: u32) -> u32 {
    const CHILD_LIST_OFFSET: u32 = 0x1D4;
    const CHILD_COUNT_OFFSET: u32 = 0x1D8;
    const KEY_METHOD_SLOT: u32 = 0x4C;
    let child_list = unsafe { ((this_ptr + CHILD_LIST_OFFSET) as *const u32).read_unaligned() };
    let child_count = unsafe { ((this_ptr + CHILD_COUNT_OFFSET) as *const u16).read_unaligned() } as u32;
    for index in 0..child_count {
        let child = unsafe { ((child_list + index * 4) as *const u32).read_unaligned() };
        let vtable = unsafe { (child as *const u32).read_unaligned() };
        let method = unsafe { ((vtable + KEY_METHOD_SLOT) as *const u32).read_unaligned() };
        let get_key: unsafe extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(method as usize) };
        if unsafe { get_key(child) } == key {
            return index;
        }
    }
    u32::MAX
});
