// original: 0x00a7f370 CSitIdleTaskInfo::vf6
/// Serialize the sit-idle values and flag byte.
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7f370(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_thiscall!(1, u32, this_, buf);
        let mut bl = (a1 & 0xFF) as u8;
        let d1 = *(this_.wrapping_add(0x18) as *const u32);
        let a2: u32 = callee_thiscall!(2, u32, buf, d1, 0x20, 0);
        bl |= (a2 & 0xFF) as u8;
        let d2 = *(this_.wrapping_add(0x1c) as *const u32);
        let a3: u32 = callee_thiscall!(3, u32, buf, d2, 0x20, 0);
        bl |= (a3 & 0xFF) as u8;
        let z = *(this_.wrapping_add(0x20) as *const u8) as u32;
        let a4: u32 = callee_thiscall!(4, u32, buf, z, 0);
        a4 | (bl as u32)
    }
});
