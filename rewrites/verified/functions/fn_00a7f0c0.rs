// original: 0x00a7f0c0 CSimpleMeleeActionResultTaskInfo::vf6
/// Serialize the melee-action-result entity handle, value and flag byte.
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7f0c0(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_thiscall!(1, u32, this_, buf);
        let mut bl = (a1 & 0xFF) as u8;
        let mut v: u32 = 0;
        let p = *(this_.wrapping_add(0x18) as *const u32);
        if p != 0 {
            let q = *(p.wrapping_add(0x6c) as *const u32);
            if q != 0 {
                let a: u32 = callee_thiscall!(2, u32, q);
                v = a & 0xFFFF;
            }
        }
        let a3: u32 = callee_thiscall!(3, u32, buf, v, 0);
        bl |= (a3 & 0xFF) as u8;
        let w = *(this_.wrapping_add(0x1c) as *const u32);
        let a4: u32 = callee_thiscall!(4, u32, buf, w, 0x20, 0);
        bl |= (a4 & 0xFF) as u8;
        let z = *(this_.wrapping_add(0x20) as *const u8) as u32;
        let a5: u32 = callee_thiscall!(5, u32, buf, z, 0);
        a5 | (bl as u32)
    }
});
