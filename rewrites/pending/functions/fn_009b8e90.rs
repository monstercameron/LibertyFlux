// original: 0x009b8e90 ui_factory_942e8_nine_fields
/// Factory for the class with vtable `0xE942E8`: nine arguments spread over
/// offsets 8..41, including one word loaded through a caller-provided pointer
/// at offset 24, two float bit patterns, and two low-byte values. The
/// original presets offset 24 to all-ones before the pointer load overwrites
/// it; the store is dead on every non-faulting path but kept here to match the
/// original's observable writes exactly. Returns null on alloc failure.
export!(stdcall, rw_009b8e90(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32, h: u32, i: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E942E8;
        let p = callee_cdecl!(1, u32, 0x2c) as *mut u8;
        if p.is_null() {
            return 0;
        }
        (p as *mut u32).write(relocated(VTABLE));
        p.add(4).write(0u8);
        (p.add(8) as *mut u32).write(a);
        (p.add(0x0c) as *mut u32).write(b);
        (p.add(0x10) as *mut u32).write(c);
        p.add(0x14).write((d & 0xFF) as u8);
        (p.add(0x18) as *mut u32).write(0xFFFFFFFF);
        (p.add(0x18) as *mut u32).write((e as *const u32).read());
        (p.add(0x1c) as *mut u32).write(f);
        (p.add(0x20) as *mut u32).write(g);
        (p.add(0x24) as *mut u32).write(h);
        p.add(0x28).write((i & 0xFF) as u8);
        p as u32
    }
});
