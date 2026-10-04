// original: 0x009b8bd0 ui_factory_94008_word_string
/// Factory for the class with vtable `0xE94008`: stores one word at offset 8
/// and copies a NUL-terminated string into the inline buffer at offset 12.
/// Returns null on alloc failure.
export!(stdcall, rw_009b8bd0(a: u32, s: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E94008;
        let p = callee_cdecl!(1, u32, 0x1c) as *mut u8;
        if p.is_null() {
            return 0;
        }
        (p as *mut u32).write(relocated(VTABLE));
        p.add(4).write(0u8);
        (p.add(8) as *mut u32).write(a);
        let mut src = s as *const u8;
        let mut dst = p.add(0x0c);
        loop {
            let c = src.read();
            dst.write(c);
            src = src.add(1);
            dst = dst.add(1);
            if c == 0 {
                break;
            }
        }
        p as u32
    }
});
