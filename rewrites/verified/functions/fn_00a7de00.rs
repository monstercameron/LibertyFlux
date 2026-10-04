// original: 0x00a7de00 CSimpleNMFlinchTaskInfo::CSimpleNMFlinchTaskInfo_2
/// Default-construct a flinch task info: copy the three default direction
/// words from the engine globals and clear the owner slot.
export!(thiscall, rw_00a7de00(this: u32) -> u32 {
    const VTABLE: u32 = 0x00EA1DCC;
    const DIR_X: u32 = 0x01B4B2A0;
    const DIR_Y: u32 = 0x01B4B2A4;
    const DIR_Z: u32 = 0x01B4B2A8;
    callee_thiscall!(2, u32, this);
    unsafe {
        let obj = this as *mut u8;
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
        (obj.add(0x20) as *mut u32).write_unaligned(global::<u32>(DIR_X).read_unaligned());
        (obj.add(0x24) as *mut u32).write_unaligned(global::<u32>(DIR_Y).read_unaligned());
        (obj.add(0x28) as *mut u32).write_unaligned(global::<u32>(DIR_Z).read_unaligned());
        (obj.add(0x30) as *mut u32).write_unaligned(0);
    }
    this
});
