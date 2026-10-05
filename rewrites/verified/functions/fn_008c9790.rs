// original: 0x008C9790 stream_slot_select (proposed)

/// Selects one of four streaming slot globals by index: 0 yields the word at
/// `SLOT0`, 1 the word at `SLOT1`, 2 the word at `SLOT2`, and any other index
/// (including large and negative values) the word at `SLOT_DEFAULT`.
///
/// One stack argument (cdecl), the selected dword returned in EAX.
lf_checker_rt::export!(cdecl, rw_008C9790(sel: u32) -> u32 {
    unsafe {
        /// Default slot word, used for any index other than 0, 1 or 2.
        const SLOT_DEFAULT: u32 = 0x1031BB8;
        /// Slot word for index 0.
        const SLOT0: u32 = 0x1031BBC;
        /// Slot word for index 1.
        const SLOT1: u32 = 0x1031BC0;
        /// Slot word for index 2.
        const SLOT2: u32 = 0x1031BC4;
        let addr = match sel {
            0 => SLOT0,
            1 => SLOT1,
            2 => SLOT2,
            _ => SLOT_DEFAULT,
        };
        (lf_checker_rt::relocated(addr) as *const u32).read()
    }
});
