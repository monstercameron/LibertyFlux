// original: 0x009b9020 ui_factory_941e8_two_words
/// Factory for the class with vtable `0xE941E8`: stores two words at offsets
/// 8 and 12. Returns null on alloc failure.
export!(stdcall, rw_009b9020(a: u32, b: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E941E8;
        let p = callee_cdecl!(1, u32, 0x10) as *mut u8;
        if p.is_null() {
            return 0;
        }
        (p as *mut u32).write(relocated(VTABLE));
        p.add(4).write(0u8);
        (p.add(8) as *mut u32).write(a);
        (p.add(0x0c) as *mut u32).write(b);
        p as u32
    }
});
