// original: 0x00a7eef0 CSequenceTaskInfo::vf6
/// Serialize the sequence-task value and signed/unsigned flag bytes.
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7eef0(this_: u32, buf: u32) -> u32 {
    unsafe {
        let v = *(this_.wrapping_add(0x14) as *const u32);
        let a1: u32 = callee_thiscall!(1, u32, buf, v, 8, 0);
        let mut bl = (a1 & 0xFF) as u8;
        let a2: u32 = callee_thiscall!(2, u32, this_, buf);
        bl |= (a2 & 0xFF) as u8;
        let s1 = *(this_.wrapping_add(0x18) as *const i8) as u32;
        let a3: u32 = callee_thiscall!(3, u32, buf, s1, 6, 0);
        bl |= (a3 & 0xFF) as u8;
        let s2 = *(this_.wrapping_add(0x19) as *const i8) as u32;
        let a4: u32 = callee_thiscall!(4, u32, buf, s2, 6, 0);
        bl |= (a4 & 0xFF) as u8;
        let z = *(this_.wrapping_add(0x1a) as *const u8) as u32;
        let a5: u32 = callee_thiscall!(5, u32, buf, z, 0);
        a5 | (bl as u32)
    }
});
