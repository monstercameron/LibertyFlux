// original: 0x009b8f50 ui_factory_93fd8_two_bytes
/// Factory for the class with vtable `0xE93FD8`: stores the low bytes of its
/// two arguments at offsets 8 and 9. Returns null on alloc failure.
export!(stdcall, rw_009b8f50(a: u32, b: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E93FD8;
        let p = callee_cdecl!(1, u32, 0x0c) as *mut u8;
        if p.is_null() {
            return 0;
        }
        (p as *mut u32).write(relocated(VTABLE));
        p.add(4).write(0u8);
        p.add(8).write((a & 0xFF) as u8);
        p.add(9).write((b & 0xFF) as u8);
        p as u32
    }
});
