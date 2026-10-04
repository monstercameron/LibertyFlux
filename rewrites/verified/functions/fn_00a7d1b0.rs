// original: 0x00a7d1b0 taskinfo_init_default
/// Runs the base initialiser, then stamps the method table, the default
/// dimensions and the fresh flag.
export!(thiscall, rw_00a7d1b0(obj: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, obj as u32);
        *((obj as *mut u8).add(0x29) as *mut u8) |= 1;
        *((obj as *mut u8) as *mut u32) = relocated(0xEA1994);
        *((obj as *mut u8).add(0x14) as *mut u32) = 0x8f;
        *((obj as *mut u8).add(0x18) as *mut u32) = 0x288;
        *((obj as *mut u8).add(0x1c) as *mut u32) = 0;
        *((obj as *mut u8).add(0x20) as *mut u32) = 0;
        *((obj as *mut u8).add(0x24) as *mut u32) = 0;
        *((obj as *mut u8).add(0x28) as *mut u8) = 0;
        obj as u32
    }
});
