// original: 0x00a31bd0 NativeImpl_IS_OBJECT_STATIC_2
/// Static/invalid-body test behind `IS_OBJECT_STATIC`.
///
/// Returns 0 when the entity's rigid-body record carries a live body index
/// (below the pool count with a clear flag entry) and 1 otherwise. Only the
/// low byte of the answer is significant: the not-found paths set AL without
/// zeroing the rest of EAX, so the checker compares just AL.
export!(thiscall, rw_00a31bd0(ent: *const u8) -> u8 {
    unsafe {
        const INVALID: u16 = 0xFFFF;
        let body = *(ent.add(0x38) as *const *const u8);
        if body.is_null() {
            return 1;
        }
        let idx = *(body.add(8) as *const u16) as u32;
        if idx == INVALID as u32 {
            return 1;
        }
        let pool = *global::<*const u8>(0x12B9C78);
        let count = *(pool.add(0x26) as *const u16) as u32;
        if idx >= count {
            return 1;
        }
        let flags = *(pool.add(0x70) as *const *const u8);
        if *flags.add(idx as usize * 8 + 4) & 3 != 0 {
            return 1;
        }
        0
    }
});
