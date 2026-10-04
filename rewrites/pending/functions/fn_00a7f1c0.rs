// original: 0x00a7f1c0 CSimpleNMFlinchTaskInfo::vf6
/// Serialize the flinch-task block pointer, word and flag byte.
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7f1c0(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_thiscall!(1, u32, this_, buf);
        let mut flag: u32 = a1 & 0xFF;
        let fp = &mut flag as *mut u32 as u32;
        let p = this_.wrapping_add(0x20);
        let _a2: u32 = callee_thiscall!(2, u32, buf, p, fp, 0x13);
        let w = *(this_.wrapping_add(0x36) as *const u16) as u32;
        let _a3: u32 = callee_thiscall!(3, u32, buf, w, fp);
        let z = *(this_.wrapping_add(0x34) as *const u8) as u32;
        let a4: u32 = callee_thiscall!(4, u32, buf, z, fp);
        (a4 & 0xFFFFFF00) | (flag & 0xFF)
    }
});
