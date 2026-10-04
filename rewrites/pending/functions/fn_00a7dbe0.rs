// original: 0x00a7dbe0 CSimpleCarDriveTaskInfo::CSimpleCarDriveTaskInfo_2
/// Default-construct a car-drive task info: base constructor plus vtable.
export!(thiscall, rw_00a7dbe0(this: u32) -> u32 {
    const VTABLE: u32 = 0x00EA13A4;
    callee_thiscall!(2, u32, this);
    unsafe {
        ((this as *mut u8) as *mut u32).write_unaligned(relocated(VTABLE));
    }
    this
});
