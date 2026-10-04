// original: 0x009b8c20 ui_factory_94048_word_float
/// Factory for the class with vtable `0xE94048`: one word plus one float bit
/// pattern at offsets 8 and 12. Returns null on alloc failure.
export!(stdcall, rw_009b8c20(a: u32, b: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E94048;
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
