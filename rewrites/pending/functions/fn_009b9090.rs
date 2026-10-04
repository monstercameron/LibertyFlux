// original: 0x009b9090 ui_factory_94208_word_byte
/// Factory for the class with vtable `0xE94208`: stores one word at offset 8
/// and the low byte of the second argument at offset 12. Returns null on
/// alloc failure.
export!(stdcall, rw_009b9090(a: u32, b: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E94208;
        let p = callee_cdecl!(1, u32, 0x10) as *mut u8;
        if p.is_null() {
            return 0;
        }
        (p as *mut u32).write(relocated(VTABLE));
        p.add(4).write(0u8);
        (p.add(8) as *mut u32).write(a);
        p.add(0x0c).write((b & 0xFF) as u8);
        p as u32
    }
});
