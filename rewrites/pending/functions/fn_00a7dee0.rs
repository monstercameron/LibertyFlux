// original: 0x00a7dee0 CSimpleSidewaysDiveTaskInfo::CSimpleSidewaysDiveTaskInfo
/// Default-construct a sideways-dive task info: zero the payload word.
export!(thiscall, rw_00a7dee0(this: u32) -> u32 {
    const VTABLE: u32 = 0x00EA1E5C;
    callee_thiscall!(2, u32, this);
    unsafe {
        let obj = this as *mut u8;
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
        (obj.add(0x18) as *mut u32).write_unaligned(0);
    }
    this
});
