// original: 0x00a7f270 CSimpleSidewaysDiveTaskInfo::vf6
/// Serialize the sideways-dive nonzero test; out-param write untestable (byte flag in scratch).
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7f270(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_thiscall!(1, u32, this_, buf);
        let mut flag: u32 = a1 & 0xFF;
        let fp = &mut flag as *mut u32 as u32;
        let d = *(this_.wrapping_add(0x18) as *const u32);
        let a2: u32 = callee_thiscall!(2, u32, buf, (d != 0) as u32, fp);
        (a2 & 0xFFFFFF00) | (flag & 0xFF)
    }
});
