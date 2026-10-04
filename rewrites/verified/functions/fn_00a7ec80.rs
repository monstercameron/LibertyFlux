// original: 0x00a7ec80 CJackTaskInfo::vf6
/// Serialize the jack-task flag byte.
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7ec80(this_: u32, buf: u32) -> u32 {
    unsafe {
        let v = *(this_.wrapping_add(0x14) as *const u8) as u32;
        callee_thiscall!(1, u32, buf, v, 0)
    }
});
