// original: 0x00DCCDE0 move_task_release_animation_reference
/// When the control task's low flag bit is set, checks the animation
/// object at offset 0xbb0. A true low-byte result triggers its release with
/// argument 250. The task pointer is the single stack argument; the method
/// returns no value and pops that word.
lf_checker_rt::export!(thiscall, rw_00dccde0(control: u32, task: u32) -> () {
    const ACTIVE_MASK: u8 = 1;
    const ANIMATION_OFFSET: usize = 0xbb0;
    const READY_ID: u32 = 1;
    const RELEASE_ID: u32 = 2;
    const RELEASE_KIND: u32 = 250;

    // SAFETY: the checker contract provides a readable control object.
    let active = unsafe { ((control as *mut u8).add(0x88)).read() & ACTIVE_MASK != 0 };
    if active {
        let animation = task.wrapping_add(ANIMATION_OFFSET as u32);
        let ready = lf_checker_rt::callee_thiscall!(READY_ID, u32, animation);
        if ready as u8 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_ID, u32, animation, RELEASE_KIND);
        }
    }
});
