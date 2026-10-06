// original: 0x008F6700 Input_IsKeyDown

/// Report whether `key` is down on this device: 1 when the byte at
/// `KEY_TABLE + (page << 8) + key` is nonzero, else 0, where `page` is the
/// global page register. All comparisons are equality only (no signedness).
///
/// When `kind` matches neither the device kind word at `+DEV_KIND_OFF` nor
/// `ANY_KIND` the function fails and returns `kind` with its low byte
/// cleared (the original only clears AL). Convention: thiscall, three stack
/// words `(key, kind, unused)`.
lf_checker_rt::export!(thiscall, rw_008f6700(this: u32, key: u32, kind: u32, _pad: u32) -> u32 {
    unsafe {
        const DEV_KIND_OFF: u32 = 4;
        const ANY_KIND: u32 = 2;
        const KEY_PAGE_REG: u32 = 0x018B7DA0;
        const KEY_TABLE: u32 = 0x018B7A90;
        let cur = (this.wrapping_add(DEV_KIND_OFF) as *const u32).read_unaligned();
        if kind != cur && kind != ANY_KIND {
            return kind & 0xFFFF_FF00;
        }
        let page = lf_checker_rt::global::<u32>(KEY_PAGE_REG).read_unaligned();
        let addr = KEY_TABLE
            .wrapping_add(page.wrapping_shl(8))
            .wrapping_add(key);
        if lf_checker_rt::global::<u8>(addr).read() != 0 {
            1
        } else {
            0
        }
    }
});
