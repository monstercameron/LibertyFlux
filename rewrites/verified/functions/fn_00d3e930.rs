// original: 0x00D3E930 task_simple_jump_launch_destroy_tail
/// Installs the launch-task vtable, removes its list node when the optional
/// linked-list pointer at offset 0x34 is non-null, and forwards to the base
/// destructor through the unconditional tail-call site. The object pointer is
/// returned by that checked tail-call stub.
lf_checker_rt::export!(thiscall, rw_d3e930(task: u32) -> u32 {
    const REMOVE_NODE: u32 = 1;
    const BASE_DESTRUCTOR: u32 = 2;
    const VTABLE: u32 = 0x00EE3DD4;
    const LINK_OFFSET: usize = 0x34;
    let object = task as *mut u8;
    unsafe { (object as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE)); }
    let link = unsafe { (object.add(LINK_OFFSET) as *const u32).read_unaligned() };
    if link != 0 {
        let _ = lf_checker_rt::callee_thiscall!(
            REMOVE_NODE, u32, link, task + LINK_OFFSET as u32
        );
    }
    lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, task)
});
