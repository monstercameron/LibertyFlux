// original: 0x00897330 aud_environment_sound_unregister_table_entry_cdecl

/// Cdecl wrapper for unregistering an environment-sound `(bank, selector)`
/// pair. The object pointer is the sole stack argument. A live selector is
/// passed to the shared manager and then replaced with 0xFF; the helper's
/// answer is returned. For selector 0xFF, only AL is defined.
export!(cdecl, rw_00897330(audio: u32) -> u32 {
    unsafe {
        const SELECTOR: u32 = 0xE8;
        const BANK: u32 = 0x40;
        const MANAGER: u32 = 0x0115_D8A0;
        const SELECTOR_NONE: u8 = 0xFF;

        let selector = (audio.wrapping_add(SELECTOR) as *const u8).read();
        if selector == SELECTOR_NONE {
            return u32::from(SELECTOR_NONE);
        }

        let bank = (audio.wrapping_add(BANK) as *const u8).read() as u32;
        let answer = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            lf_checker_rt::relocated(MANAGER),
            bank,
            u32::from(selector)
        );
        (audio.wrapping_add(SELECTOR) as *mut u8).write(SELECTOR_NONE);
        answer
    }
});
