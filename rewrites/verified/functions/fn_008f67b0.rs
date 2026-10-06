// original: 0x008F67B0 Input_SetKeyPressed

/// Mark `key` pressed: store 1 at `KEY_TABLE[(page << 8) + key]`, 0x80 at
/// `KEY_TABLE3[key]`, and 0 at `KEY_TABLE[((page ^ 1) << 8) + key]`, where
/// `page` is the global page register. Returns `(page << 8) | 1` (the
/// original only writes AL over the shifted page).
///
/// When `kind` matches neither the device kind word nor `ANY_KIND` nothing
/// is stored and the function returns `kind` with its low byte cleared.
/// Convention: thiscall, three stack words `(key, kind, unused)`.
lf_checker_rt::export!(thiscall, rw_008f67b0(this: u32, key: u32, kind: u32, _pad: u32) -> u32 {
    unsafe {
        const DEV_KIND_OFF: u32 = 4;
        const ANY_KIND: u32 = 2;
        const KEY_PAGE_REG: u32 = 0x018B7DA0;
        const KEY_TABLE: u32 = 0x018B7A90;
        const KEY_TABLE3: u32 = 0x018B7C90;
        let cur = (this.wrapping_add(DEV_KIND_OFF) as *const u32).read_unaligned();
        if kind != cur && kind != ANY_KIND {
            return kind & 0xFFFF_FF00;
        }
        let page = lf_checker_rt::global::<u32>(KEY_PAGE_REG).read_unaligned();
        let here = KEY_TABLE
            .wrapping_add(page.wrapping_shl(8))
            .wrapping_add(key);
        lf_checker_rt::global::<u8>(here).write(1);
        lf_checker_rt::global::<u8>(KEY_TABLE3.wrapping_add(key)).write(0x80);
        let other = KEY_TABLE
            .wrapping_add((page ^ 1).wrapping_shl(8))
            .wrapping_add(key);
        lf_checker_rt::global::<u8>(other).write(0);
        page.wrapping_shl(8) | 1
    }
});
