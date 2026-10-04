// original: 0x009b8b90 ui_factory_94088_word_float
/// Factory for the class with vtable `0xE94088`: stores one word and one
/// float (copied as raw bits) at offsets 8 and 12. Returns null on alloc
/// failure.
export!(stdcall, rw_009b8b90(a: u32, b: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E94088;
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
