// original: 0x00a7f2e0 CSitDownIdleThenStandTaskInfo::vf6
/// Serialize the sit-down values, blend float (forwarded as bits) and flag bytes.
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7f2e0(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_thiscall!(1, u32, this_, buf);
        let mut bl = (a1 & 0xFF) as u8;
        let d1 = *(this_.wrapping_add(0x18) as *const u32);
        let a2: u32 = callee_thiscall!(2, u32, buf, d1, 3, 0);
        bl |= (a2 & 0xFF) as u8;
        let d2 = *(this_.wrapping_add(0x1c) as *const u32);
        let a3: u32 = callee_thiscall!(3, u32, buf, d2, 3, 0);
        bl |= (a3 & 0xFF) as u8;
        let fb = *(this_.wrapping_add(0x20) as *const u32);
        let a4: u32 = callee_thiscall!(4, u32, buf, fb, 0x40C90FDB, 8, 0);
        bl |= (a4 & 0xFF) as u8;
        let z1 = *(this_.wrapping_add(0x24) as *const u8) as u32;
        let a5: u32 = callee_thiscall!(5, u32, buf, z1, 0);
        bl |= (a5 & 0xFF) as u8;
        let z2 = *(this_.wrapping_add(0x25) as *const u8) as u32;
        let a6: u32 = callee_thiscall!(6, u32, buf, z2, 0);
        bl |= (a6 & 0xFF) as u8;
        let p = this_.wrapping_add(0x30);
        let a7: u32 = callee_thiscall!(7, u32, buf, p, 0, 0x13);
        a7 | (bl as u32)
    }
});
