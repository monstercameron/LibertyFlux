// original: 0x00a7dd80 CSimpleNMExplosionTaskInfo::CSimpleNMExplosionTaskInfo_2
/// Default-construct an explosion task info: base constructor plus vtable.
export!(thiscall, rw_00a7dd80(this: u32) -> u32 {
    const VTABLE: u32 = 0x00EA1C64;
    callee_thiscall!(2, u32, this);
    unsafe {
        ((this as *mut u8) as *mut u32).write_unaligned(relocated(VTABLE));
    }
    this
});
