// original: 0x00a7f3c0 CStandUpTaskInfo::vf6
/// Serialize the stand-up values.
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7f3c0(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_thiscall!(1, u32, this_, buf);
        let mut bl = (a1 & 0xFF) as u8;
        let d1 = *(this_.wrapping_add(0x18) as *const u32);
        let a2: u32 = callee_thiscall!(2, u32, buf, d1, 0x10, 0);
        bl |= (a2 & 0xFF) as u8;
        let d2 = *(this_.wrapping_add(0x1c) as *const u32);
        let a3: u32 = callee_thiscall!(3, u32, buf, d2, 0x10, 0);
        a3 | (bl as u32)
    }
});
