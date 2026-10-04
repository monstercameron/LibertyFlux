// original: 0x00a7ed30 CMeleeTaskInfo::vf6
/// Serialize the melee-task value.
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7ed30(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_thiscall!(1, u32, this_, buf);
        let v = *(this_.wrapping_add(0x1c) as *const u32);
        let a2: u32 = callee_thiscall!(2, u32, buf, v, 0x20, 0);
        a2 | (a1 & 0xFF)
    }
});
