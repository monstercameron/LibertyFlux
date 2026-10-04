// original: 0x009b8f80 ui_factory_94178_words_floats
/// Factory for the class with vtable `0xE94178`: four words at offsets 8..20
/// followed by three float bit patterns at offsets 24..36. Returns null on
/// alloc failure.
export!(stdcall, rw_009b8f80(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E94178;
        let p = callee_cdecl!(1, u32, 0x24) as *mut u8;
        if p.is_null() {
            return 0;
        }
        (p as *mut u32).write(relocated(VTABLE));
        p.add(4).write(0u8);
        (p.add(8) as *mut u32).write(a);
        (p.add(0x0c) as *mut u32).write(b);
        (p.add(0x10) as *mut u32).write(c);
        (p.add(0x14) as *mut u32).write(d);
        (p.add(0x18) as *mut u32).write(e);
        (p.add(0x1c) as *mut u32).write(f);
        (p.add(0x20) as *mut u32).write(g);
        p as u32
    }
});
