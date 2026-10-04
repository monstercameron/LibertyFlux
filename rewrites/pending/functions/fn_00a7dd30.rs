// original: 0x00a7dd30 CSimpleNMBraceTaskInfo::CSimpleNMBraceTaskInfo_2
/// Default-construct a brace task info: null owner.
export!(thiscall, rw_00a7dd30(this: u32) -> u32 {
    const VTABLE: u32 = 0x00EA1BD4;
    callee_thiscall!(2, u32, this);
    unsafe {
        let obj = this as *mut u8;
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
        (obj.add(0x18) as *mut u32).write_unaligned(0);
    }
    this
});
