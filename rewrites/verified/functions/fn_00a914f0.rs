// original: 0x00a914f0 stream_word_install

/// Installs a global word into a looked-up record, when enabled.
///
/// When the flag byte at `this+0x76` is clear, returns the incoming EAX
/// (fixed to 0 by the contract: Rust cannot read it). Otherwise calls the
/// lookup (callee 1, thiscall) with (`a`, `b`); a null answer is returned
/// as-is, while a live pointer gets the global word at file VA 0x11A8908
/// (zero-extended) stored at `+0x3C` and is returned. One call.
/// Original: 0x00A914F0 (thiscall, ECX + two stack words), 36 bytes.
lf_checker_rt::export!(thiscall, rw_00a914f0(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x76;
        const SLOT_OFF: u32 = 0x3C;
        const WORD_GLOBAL: u32 = 0x11A8908;
        const LOOKUP: u32 = 1;
        if (this.wrapping_add(FLAG_OFF) as *const u8).read() == 0 {
            return 0;
        }
        let found: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32, this, a, b);
        if found == 0 {
            return 0;
        }
        let word =
            (lf_checker_rt::global::<u16>(WORD_GLOBAL) as *const u16).read_unaligned() as u32;
        (found.wrapping_add(SLOT_OFF) as *mut u32).write_unaligned(word);
        found
    }
});
