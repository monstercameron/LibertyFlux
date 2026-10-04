// original: 0x00a7dc50 CSimpleMeleeActionResultTaskInfo::CSimpleMeleeActionResultTaskInfo
/// Construct a melee-result task info: store the target pointer, an extra
/// word and a kind byte, then register the target slot with the engine
/// list helper unless the target is null.
export!(thiscall, rw_00a7dc50(this: u32, target: u32, extra: u32, kind: u32) -> u32 {
    const VTABLE: u32 = 0x00EA1284;
    callee_thiscall!(2, u32, this);
    unsafe {
        let obj = this as *mut u8;
        (obj.add(0x1c) as *mut u32).write_unaligned(extra);
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
        (obj.add(0x18) as *mut u32).write_unaligned(target);
        *obj.add(0x20) = kind as u8;
    }
    if target != 0 {
        callee_thiscall!(3, u32, target, this.wrapping_add(0x18));
    }
    this
});
