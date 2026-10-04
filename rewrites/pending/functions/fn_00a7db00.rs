// original: 0x00a7db00 CSimpleAimPlayerProjectileInfo::CSimpleAimPlayerProjectileInfo
/// Construct an aim-projectile task info holding one flag byte.
export!(thiscall, rw_00a7db00(this: u32, flags: u32) -> u32 {
    const VTABLE: u32 = 0x00EA1B44;
    callee_thiscall!(2, u32, this);
    unsafe {
        let obj = this as *mut u8;
        *obj.add(0x18) = flags as u8;
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
    }
    this
});
