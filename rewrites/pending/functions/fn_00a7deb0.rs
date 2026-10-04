// original: 0x00a7deb0 CSimpleNMShotTaskInfo::CSimpleNMShotTaskInfo_2
/// Default-construct a shot task info: zero words and null owner.
export!(thiscall, rw_00a7deb0(this: u32) -> u32 {
    const VTABLE: u32 = 0x00EA1C1C;
    callee_thiscall!(2, u32, this);
    unsafe {
        let obj = this as *mut u8;
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
        (obj.add(0x18) as *mut u32).write_unaligned(0);
        (obj.add(0x1c) as *mut u32).write_unaligned(0);
        (obj.add(0x20) as *mut u32).write_unaligned(0);
    }
    this
});
