// original: 0x009b8e60 ui_factory_946b8_word
/// Factory for the class with vtable `0xE946B8`: stores its single word
/// argument at offset 8. Returns null on alloc failure.
export!(stdcall, rw_009b8e60(a: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E946B8;
        let p = callee_cdecl!(1, u32, 0x0c) as *mut u8;
        if p.is_null() {
            return 0;
        }
        (p as *mut u32).write(relocated(VTABLE));
        p.add(4).write(0u8);
        (p.add(8) as *mut u32).write(a);
        p as u32
    }
});
