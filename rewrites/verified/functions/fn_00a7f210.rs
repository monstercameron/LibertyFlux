// original: 0x00a7f210 CSimpleNMShotTaskInfo::vf6
/// Serialize the shot-task values, word and flag byte.
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7f210(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_thiscall!(1, u32, this_, buf);
        let mut flag: u32 = a1 & 0xFF;
        let fp = &mut flag as *mut u32 as u32;
        let d1 = *(this_.wrapping_add(0x18) as *const u32);
        let _a2: u32 = callee_thiscall!(2, u32, buf, d1, 6, fp);
        let d2 = *(this_.wrapping_add(0x1c) as *const u32);
        let _a3: u32 = callee_thiscall!(3, u32, buf, d2, 7, fp);
        let w = *(this_.wrapping_add(0x26) as *const u16) as u32;
        let _a4: u32 = callee_thiscall!(4, u32, buf, w, fp);
        let z = *(this_.wrapping_add(0x24) as *const u8) as u32;
        let a5: u32 = callee_thiscall!(5, u32, buf, z, fp);
        (a5 & 0xFFFFFF00) | (flag & 0xFF)
    }
});
