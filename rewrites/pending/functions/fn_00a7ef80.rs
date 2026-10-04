// original: 0x00a7ef80 CSimpleCarDriveTaskInfo::vf6
/// Serialize the six car-drive flag bits.
///
/// Thiscall serializer: ECX is the task-info object, the stack
/// argument is the network buffer object; every engine helper is
/// intercepted by the checker and answered by script.
export!(thiscall, rw_00a7ef80(this_: u32, buf: u32) -> u32 {
    unsafe {
        let a0: u32 = callee_thiscall!(1, u32, this_, buf);
        let mut flag: u32 = a0 & 0xFF;
        let fp = &mut flag as *mut u32 as u32;
        let bits = *(this_.wrapping_add(0x1c) as *const u32);
        let _a1: u32 = callee_thiscall!(2, u32, buf, bits & 1, fp);
        let _a2: u32 = callee_thiscall!(3, u32, buf, (bits >> 1) & 1, fp);
        let _a3: u32 = callee_thiscall!(4, u32, buf, (bits >> 2) & 1, fp);
        let _a4: u32 = callee_thiscall!(5, u32, buf, (bits >> 3) & 1, fp);
        let _a5: u32 = callee_thiscall!(6, u32, buf, (bits >> 4) & 1, fp);
        let a6: u32 = callee_thiscall!(7, u32, buf, (bits >> 5) & 1, fp);
        (a6 & 0xFFFFFF00) | (flag & 0xFF)
    }
});
