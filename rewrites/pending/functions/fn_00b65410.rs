// original: 0x00b65410 NativeImpl_FORCE_CHAR_TO_DROP_WEAPON_3
// stdcall/1 leaf (FORCE_CHAR_TO_DROP_WEAPON_3 native impl). Reports whether
// the given record is present, tagged 0x2e and carries status 2. The verdict
// byte is merged over the record pointer's own high bytes.
export!(stdcall, rw_rs11f17(record: u32) -> u32 {
    unsafe {
        if record == 0 {
            return 0;
        }
        let linked = *((record + 0x25c) as *const u32);
        let ok = linked != 0
            && *((linked + 0x18) as *const u32) == 0x2e
            && *((record + 0x22a) as *const u8) == 2;
        (record & !0xff) | (ok as u32)
    }
});
