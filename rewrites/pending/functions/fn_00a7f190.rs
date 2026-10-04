// original: 0x00a7f190 CSimpleNMExplosionTaskInfo::vf6
/// Serialize the explosion-task block pointer; out-param write untestable (byte flag in scratch).
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7f190(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_thiscall!(1, u32, this_, buf);
        let mut flag: u32 = a1 & 0xFF;
        let fp = &mut flag as *mut u32 as u32;
        let p = this_.wrapping_add(0x20);
        let a2: u32 = callee_thiscall!(2, u32, buf, p, fp, 0x13);
        (a2 & 0xFFFFFF00) | (flag & 0xFF)
    }
});
