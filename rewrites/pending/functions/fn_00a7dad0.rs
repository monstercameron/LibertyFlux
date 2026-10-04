// original: 0x00a7dad0 CSequenceTaskInfo::CSequenceTaskInfo_2
/// Default-construct a sequence task info: base constructor, vtable, zero
/// step count and flags, and the trailing mode byte set to 1.
export!(thiscall, rw_00a7dad0(this: u32) -> u32 {
    const VTABLE: u32 = 0x00EA12CC;
    callee_thiscall!(2, u32, this);
    unsafe {
        let obj = this as *mut u8;
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
        (obj.add(0x14) as *mut u32).write_unaligned(0);
        (obj.add(0x18) as *mut u16).write_unaligned(0);
        *obj.add(0x1a) = 1;
    }
    this
});
