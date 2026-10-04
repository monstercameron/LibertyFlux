// original: 0x00E6F590 timing_link_reset
/// Reset the timing link slot.
///
/// Reads the link word; if it still points at the flag slot it is cleared,
/// otherwise it is kept. Then the flag slot is set to its reset marker and
/// the link word is written back. Returns the link value that was kept.
export!(cdecl, rw_00e6f590() -> u32 {
    unsafe {
        const LINK_SLOT: u32 = 0x018B7A60;
        const FLAG_SLOT: u32 = 0x019F1418;
        const FLAG_PTR: u32 = 0x00FE37A0;
        let mut link = *global::<u32>(LINK_SLOT);
        if link == relocated(FLAG_SLOT) {
            link = 0;
        }
        *global::<u32>(FLAG_SLOT) = relocated(FLAG_PTR);
        *global::<u32>(LINK_SLOT) = link;
        link
    }
});
