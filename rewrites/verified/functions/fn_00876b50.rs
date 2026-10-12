// original: 0x00876B50 rage::crmtRequestFrame::vf1

/// Reset the request frame and release its attached child frame.
///
/// The frame's reset word at `+0x14` is cleared on every path. If the child
/// pointer at `+0x0c` is non-null, the child detach helper receives that child
/// as its this pointer and the address of the parent link at `this + 4` as its
/// one stack argument. The helper's result is returned; the null path returns
/// zero from the child-pointer load.
///
/// Original calling convention: thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00876b50(this: u32) -> u32 {
    const CHILD_OFFSET: u32 = 0x0c;
    const RESET_WORD_OFFSET: u32 = 0x14;
    const PARENT_LINK_OFFSET: u32 = 0x04;
    unsafe {
        (this.wrapping_add(RESET_WORD_OFFSET) as *mut u32).write_unaligned(0);
        let child = (this.wrapping_add(CHILD_OFFSET) as *const u32).read_unaligned();
        if child == 0 {
            0
        } else {
            let parent_link = this.wrapping_add(PARENT_LINK_OFFSET);
            lf_checker_rt::callee_thiscall!(1, u32, child, parent_link)
        }
    }
});
