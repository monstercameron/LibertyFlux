// original: 0x008F6990 GetCurrentInputObject

/// Return the current input object: `OBJ_CURRENT` when the index global
/// names a live table entry and either the low byte of `flag` is 0 or that
/// entry's state dword at `+STATE_OFF` is 0; otherwise `OBJ_DEFAULT`.
///
/// The index `-1`, a null entry, a nonzero flag byte with nonzero state all
/// select the default. All comparisons are equality only. Convention: cdecl,
/// one stack word (only its low byte is read).
lf_checker_rt::export!(cdecl, rw_008f6990(flag: u32) -> u32 {
    unsafe {
        const INDEX_REG: u32 = 0x01036F14;
        const OBJ_TABLE: u32 = 0x011A8808;
        const STATE_OFF: u32 = 0x4C8;
        const OBJ_CURRENT: u32 = 0x0117E700;
        const OBJ_DEFAULT: u32 = 0x01185C08;
        let idx = lf_checker_rt::global::<u32>(INDEX_REG).read_unaligned();
        if idx != 0xFFFF_FFFF {
            let entry = lf_checker_rt::global::<u32>(
                OBJ_TABLE.wrapping_add(idx.wrapping_mul(4)),
            )
            .read_unaligned();
            if entry != 0 {
                if flag & 0xFF == 0 {
                    return lf_checker_rt::relocated(OBJ_CURRENT);
                }
                let state = (entry.wrapping_add(STATE_OFF) as *const u32)
                    .read_unaligned();
                if state == 0 {
                    return lf_checker_rt::relocated(OBJ_CURRENT);
                }
            }
        }
        lf_checker_rt::relocated(OBJ_DEFAULT)
    }
});
