// original: 0x00a7f030 CSimpleClimbTaskInfo::vf6
/// Serialize the climb-task presence flags; object address itself is tested.
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7f030(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_thiscall!(1, u32, this_, buf);
        let mut bl = (a1 & 0xFF) as u8;
        let c0 = (this_.wrapping_add(0x18) != 0) as u32;
        let a2: u32 = callee_thiscall!(2, u32, buf, c0, 0);
        bl |= (a2 & 0xFF) as u8;
        let c1 = (this_.wrapping_add(0x19) != 0) as u32;
        let a3: u32 = callee_thiscall!(3, u32, buf, c1, 0);
        bl |= (a3 & 0xFF) as u8;
        let c2 = (this_.wrapping_add(0x1a) != 0) as u32;
        let a4: u32 = callee_thiscall!(4, u32, buf, c2, 0);
        bl |= (a4 & 0xFF) as u8;
        let c3 = (this_.wrapping_add(0x1b) != 0) as u32;
        let a5: u32 = callee_thiscall!(5, u32, buf, c3, 0);
        bl |= (a5 & 0xFF) as u8;
        let p = this_.wrapping_add(0x20);
        let a6: u32 = callee_thiscall!(6, u32, buf, p, 0, 0x13);
        a6 | (bl as u32)
    }
});
