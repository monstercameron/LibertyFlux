// original: 0x009b8da0 ui_factory_94578_byte
/// Factory for the class with vtable `0xE94578`: stores the low byte of its
/// single argument at offset 8. Returns null on alloc failure.
export!(stdcall, rw_009b8da0(a: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E94578;
        let p = callee_cdecl!(1, u32, 0x0c) as *mut u8;
        if p.is_null() {
            return 0;
        }
        (p as *mut u32).write(relocated(VTABLE));
        p.add(4).write(0u8);
        p.add(8).write((a & 0xFF) as u8);
        p as u32
    }
});
