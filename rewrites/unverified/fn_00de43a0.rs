// original: 0x00DE43A0 UIRowLayout::vf139

/// Use the first incoming stack word as the child selection. If the child's mode-selected readiness method returns nonzero, prepare the current child, store the selection in the parent's state word at `+0x1F4`, and commit the current child. The readiness method is selected by the parent's byte flag at `+0x1F0`; the second incoming stack word is unused.
lf_checker_rt::export!(thiscall, rw_00de43a0(this_ptr: u32, selected_child: u32, _unused_argument: u32) -> u32 {
    const INDEX_SLOT: u32 = 0x21C;
    const CHILD_SLOT: u32 = 0x1E0;
    const READY_LOW_SLOT: u32 = 0x0C;
    const READY_HIGH_SLOT: u32 = 0x10;
    const PREPARE_SLOT: u32 = 0x1B0;
    const COMMIT_SLOT: u32 = 0x1AC;
    const PARENT_STATE: u32 = 0x1F4;
    const PARENT_FLAG: u32 = 0x1F0;
    let parent_vtable = unsafe { (this_ptr as *const u32).read_unaligned() };
    let current_index_ptr = unsafe { ((parent_vtable + INDEX_SLOT) as *const u32).read_unaligned() };
    let current_index: unsafe extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(current_index_ptr as usize) };
    let child_getter_ptr = unsafe { ((parent_vtable + CHILD_SLOT) as *const u32).read_unaligned() };
    let child_getter: unsafe extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(child_getter_ptr as usize) };
    let child = unsafe { child_getter(this_ptr, selected_child) };
    if child == 0 { return 0; }
    let child_vtable = unsafe { (child as *const u32).read_unaligned() };
    let flag = unsafe { ((this_ptr + PARENT_FLAG) as *const u8).read() };
    let ready_slot = if flag != 0 { READY_HIGH_SLOT } else { READY_LOW_SLOT };
    let ready_ptr = unsafe { ((child_vtable + ready_slot) as *const u32).read_unaligned() };
    let is_ready: unsafe extern "thiscall" fn(u32) -> u8 = unsafe { core::mem::transmute(ready_ptr as usize) };
    if unsafe { is_ready(child) } == 0 { return 0; }
    let index = unsafe { current_index(this_ptr) };
    let child = unsafe { child_getter(this_ptr, index) };
    let child_vtable = unsafe { (child as *const u32).read_unaligned() };
    let prepare_ptr = unsafe { ((child_vtable + PREPARE_SLOT) as *const u32).read_unaligned() };
    let prepare: unsafe extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(prepare_ptr as usize) };
    let _prepare_result = unsafe { prepare(child) };
    unsafe { ((this_ptr + PARENT_STATE) as *mut u32).write_unaligned(selected_child) };
    let index = unsafe { current_index(this_ptr) };
    let child = unsafe { child_getter(this_ptr, index) };
    let child_vtable = unsafe { (child as *const u32).read_unaligned() };
    let commit_ptr = unsafe { ((child_vtable + COMMIT_SLOT) as *const u32).read_unaligned() };
    let commit: unsafe extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(commit_ptr as usize) };
    let _commit_result = unsafe { commit(child) };
    0
});
