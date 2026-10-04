// original: 0x009b78e0 NativeImpl_IS_CAM_ACTIVE
/// Look up the camera object for `key` and test bit 2 of the flag byte at
/// +0x13c. The result rides in AL; the upper bytes are the lookup answer's
/// own high bytes, exactly as the original leaves them.
/// (Engine predicate behind IS_CAM_ACTIVE.)
export!(stdcall, rw_009b78e0(key: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x13C;
        const BIT: u32 = 2;
        let obj = callee_stdcall!(1, u32, key);
        let flags = *((obj.wrapping_add(FLAG_OFF)) as *const u8);
        (obj & 0xFFFF_FF00) | (((flags >> BIT) & 1) as u32)
    }
});
