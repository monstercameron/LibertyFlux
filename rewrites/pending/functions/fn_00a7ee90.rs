// original: 0x00a7ee90 CScriptAnimTaskInfo::vf6
/// Serialize the script-anim-task values; out-param writes untestable (byte flag in scratch).
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7ee90(this_: u32, buf: u32) -> u32 {
    unsafe {
        let mut flag: u32 = 0;
        let fp = &mut flag as *mut u32 as u32;
        let v1 = *(this_.wrapping_add(0x14) as *const u32);
        let _a1: u32 = callee_thiscall!(1, u32, buf, v1, 0x0c, fp);
        let v2 = *(this_.wrapping_add(0x18) as *const u32);
        let _a2: u32 = callee_thiscall!(2, u32, buf, v2, 0x20, fp);
        let _a3: u32 = callee_thiscall!(3, u32, buf, 8, 1);
        let _a4: u32 = callee_thiscall!(4, u32, buf, 3, 1);
        let a5: u32 = callee_thiscall!(5, u32, buf, 0x1f, 1);
        (a5 & 0xFFFFFF00) | (flag & 0xFF)
    }
});
