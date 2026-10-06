// original: 0x008F6730 Input_IsKeyJustDown

/// Report whether `key` just went down: 1 when
/// `KEY_TABLE[(page << 8) + key] & (KEY_TABLE2[key] ^ KEY_TABLE[key])` is
/// nonzero, else 0, where `page` is the global page register. Equality only.
///
/// When `kind` matches neither the device kind word nor `ANY_KIND` the
/// function fails and returns `kind` with its low byte cleared. Convention:
/// thiscall, three stack words `(key, kind, unused)`.
lf_checker_rt::export!(thiscall, rw_008f6730(this: u32, key: u32, kind: u32, _pad: u32) -> u32 {
    unsafe {
        const DEV_KIND_OFF: u32 = 4;
        const ANY_KIND: u32 = 2;
        const KEY_PAGE_REG: u32 = 0x018B7DA0;
        const KEY_TABLE: u32 = 0x018B7A90;
        const KEY_TABLE2: u32 = 0x018B7B90;
        let cur = (this.wrapping_add(DEV_KIND_OFF) as *const u32).read_unaligned();
        if kind != cur && kind != ANY_KIND {
            return kind & 0xFFFF_FF00;
        }
        let page = lf_checker_rt::global::<u32>(KEY_PAGE_REG).read_unaligned();
        let prev = lf_checker_rt::global::<u8>(KEY_TABLE2.wrapping_add(key)).read();
        let cur_b = lf_checker_rt::global::<u8>(KEY_TABLE.wrapping_add(key)).read();
        let addr = KEY_TABLE
            .wrapping_add(page.wrapping_shl(8))
            .wrapping_add(key);
        let gated = lf_checker_rt::global::<u8>(addr).read();
        if gated & (prev ^ cur_b) != 0 {
            1
        } else {
            0
        }
    }
});
