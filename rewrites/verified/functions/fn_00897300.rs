// original: 0x00897300 aud_environment_sound_unregister_table_entry

/// If the object's selector at `+0xE8` is live, ask the shared manager to
/// unregister the `(bank, selector)` pair and then mark the selector unused.
/// The thiscall helper's EAX result is returned. A selector of 0xFF skips the
/// call; only AL is defined on that path.
export!(thiscall, rw_00897300(audio: u32) -> u32 {
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
