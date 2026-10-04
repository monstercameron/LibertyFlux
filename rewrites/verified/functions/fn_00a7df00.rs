// original: 0x00a7df00 CSimpleThrowProjectileInfo::CSimpleThrowProjectileInfo
/// Construct a throw-projectile info holding one word.
export!(thiscall, rw_00a7df00(this: u32, value: u32) -> u32 {
    const VTABLE: u32 = 0x00EA1B8C;
    callee_thiscall!(2, u32, this);
    unsafe {
        let obj = this as *mut u8;
        (obj.add(0x18) as *mut u32).write_unaligned(value);
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
    }
    this
});
