// original: 0x00a7ef50 CPlayRandomAmbientsInfo::vf6
/// Serialize the play-random-ambients flag byte.
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7ef50(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_thiscall!(1, u32, this_, buf);
        let v = *(this_.wrapping_add(0x18) as *const u8) as u32;
        let a2: u32 = callee_thiscall!(2, u32, buf, v, 0);
        a2 | (a1 & 0xFF)
    }
});
