// original: 0x00a7eca0 CMedicTaskInfo::vf6
/// Serialize the medic-task entity handles and flag bit.
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7eca0(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_thiscall!(1, u32, this_, buf);
        let mut bl = (a1 & 0xFF) as u8;
        let mut v1: u32 = 0;
        let p1 = *(this_.wrapping_add(0x1c) as *const u32);
        if p1 != 0 {
            let q1 = *(p1.wrapping_add(0x6c) as *const u32);
            if q1 != 0 {
                let a: u32 = callee_thiscall!(2, u32, q1);
                v1 = a & 0xFFFF;
            }
        }
        let mut v2: u32 = 0;
        let p2 = *(this_.wrapping_add(0x20) as *const u32);
        if p2 != 0 {
            let q2 = *(p2.wrapping_add(0x6c) as *const u32);
            if q2 != 0 {
                let a: u32 = callee_thiscall!(3, u32, q2);
                v2 = a & 0xFFFF;
            }
        }
        let a2: u32 = callee_thiscall!(4, u32, buf, v1, 0);
        bl |= (a2 & 0xFF) as u8;
        let a3: u32 = callee_thiscall!(5, u32, buf, v2, 0);
        bl |= (a3 & 0xFF) as u8;
        let b = (*(this_.wrapping_add(0x24) as *const u8) & 1) as u32;
        let a4: u32 = callee_thiscall!(6, u32, buf, b, 0);
        a4 | (bl as u32)
    }
});
