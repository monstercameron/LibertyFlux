// original: 0x00AF78B0 veh_slots_remap_range (proposed)

/// Remap slot entries from `start` upwards through the slot translator.
///
/// For each index `i` from `start` to 12: when either word of the dword pair
/// at `this + 4 * i` holds the empty marker `0xFFFF`, the word at
/// `this + 0x38 + 2 * i` is set to the marker; otherwise callee 1 translates
/// the two dwords and its low word is stored there. A `start` of 13 or more
/// does nothing. Nothing is returned.
///
/// Original: 0x00AF78B0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00AF78B0(this: u32, start: u32) -> u32 {
    unsafe {
        const TRANSLATE: u32 = 1;
        const TRANSLATOR: u32 = 0x1177A80;
        let translator = lf_checker_rt::relocated(TRANSLATOR);
        const TOP: u32 = 13;
        const OUT: u32 = 0x38;
        const EMPTY: u16 = 0xFFFF;
        if start >= TOP {
            return 0;
        }
        for i in start..TOP {
            let lo = ((this + i * 4) as *const u16).read_unaligned();
            let hi = ((this + 4 + i * 4) as *const u16).read_unaligned();
            let out = (this + OUT + i * 2) as *mut u16;
            if lo == EMPTY || hi == EMPTY {
                out.write_unaligned(EMPTY);
            } else {
                let a = ((this + i * 4) as *const u32).read_unaligned();
                let b = ((this + 4 + i * 4) as *const u32).read_unaligned();
                let v = lf_checker_rt::callee_thiscall!(TRANSLATE, u32, translator, a, b);
                out.write_unaligned(v as u16);
            }
        }
        0
    }
});
