// original: 0x00DE4500 UIRowLayout::vf108

/// Read the parent's current child index and resolve that child. When it exists, call its preparation method at `+0x1B0`, then its enable method at `+0x18` with zero. Tail-forward the parent to the direct continuation and return that call's 32-bit result. The child calls use `thiscall`; the enable call consumes one stack word.
lf_checker_rt::export!(thiscall, rw_00de4500(this_ptr: u32) -> u32 {
    const INDEX_SLOT: u32 = 0x21C;
    const CHILD_SLOT: u32 = 0x1E0;
    const PREPARE_SLOT: u32 = 0x1B0;
    const ENABLE_SLOT: u32 = 0x18;
    let parent_vtable = unsafe { (this_ptr as *const u32).read_unaligned() };
    let index_method_ptr = unsafe { ((parent_vtable + INDEX_SLOT) as *const u32).read_unaligned() };
    let index_method: unsafe extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(index_method_ptr as usize) };
    let index = unsafe { index_method(this_ptr) };
    let child_method_ptr = unsafe { ((parent_vtable + CHILD_SLOT) as *const u32).read_unaligned() };
    let child_method: unsafe extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(child_method_ptr as usize) };
    let child = unsafe { child_method(this_ptr, index) };
    if child != 0 {
        let child_vtable = unsafe { (child as *const u32).read_unaligned() };
        let prepare_ptr = unsafe { ((child_vtable + PREPARE_SLOT) as *const u32).read_unaligned() };
        let prepare: unsafe extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(prepare_ptr as usize) };
        let _prepare_result = unsafe { prepare(child) };
        let enable_ptr = unsafe { ((child_vtable + ENABLE_SLOT) as *const u32).read_unaligned() };
        let enable: unsafe extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(enable_ptr as usize) };
        let _enable_result = unsafe { enable(child, 0) };
    }
    lf_checker_rt::callee_thiscall!(5, u32, this_ptr)
});
