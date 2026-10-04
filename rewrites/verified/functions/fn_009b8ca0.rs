// original: 0x009b8ca0 ui_factory_94018_word_record
/// Factory for the class with vtable `0xE94018`: stores one word at offset 8
/// and copies four words (a word, two float bit patterns, a word) from a
/// caller-provided record into offsets 16..32. Returns null on alloc failure.
export!(stdcall, rw_009b8ca0(a: u32, q: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E94018;
        let p = callee_cdecl!(1, u32, 0x20) as *mut u8;
        if p.is_null() {
            return 0;
        }
        let src = q as *const u32;
        (p as *mut u32).write(relocated(VTABLE));
        p.add(4).write(0u8);
        (p.add(8) as *mut u32).write(a);
        (p.add(0x10) as *mut u32).write(src.read());
        (p.add(0x14) as *mut u32).write(src.add(1).read());
        (p.add(0x18) as *mut u32).write(src.add(2).read());
        (p.add(0x1c) as *mut u32).write(src.add(3).read());
        p as u32
    }
});
