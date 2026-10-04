// original: 0x00a7f400 CStatusAndTargetTaskInfo::vf6
/// Serialize the status-and-target word.
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7f400(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_thiscall!(1, u32, this_, buf);
        let w = *(this_.wrapping_add(0x20) as *const u16) as u32;
        let a2: u32 = callee_thiscall!(2, u32, buf, w, 0);
        a2 | (a1 & 0xFF)
    }
});
