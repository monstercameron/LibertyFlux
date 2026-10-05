// original: 0x00b98ea0 NativeImpl_IS_KEYBOARD_KEY_JUST_PRESSED

/// Returns whether a keyboard key was just pressed, as a normalized flag.
///
/// Calls the input method `POLL` on the global input object `OBJ` with
/// (`key`, 0, `TAG`), where `key` is the caller's key code. The result is
/// normalized from the callee's low byte only: bit 0 is set when the low
/// byte is nonzero, while the upper three bytes of the callee's return
/// value pass through unchanged. `bl` is zeroed before the call and read
/// back after it, which relies on the callee preserving `ebx` (it is
/// callee-saved, and the checker stub preserves it too).
///
/// The pushed object pointer and tag address are absolute immediates with
/// relocation entries, so both sides push the relocated addresses and the
/// call log compares them as image-relative offsets.
///
/// Original: 0x00B98EA0 (cdecl, one stack word, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b98ea0(key: u32) -> u32 {
    const OBJ: u32 = 0x0118D110;
    const TAG: u32 = 0x00EB5CD0;
    const POLL: u32 = 1;
    let r: u32 = lf_checker_rt::callee_thiscall!(
        POLL,
        u32,
        lf_checker_rt::relocated(OBJ),
        key,
        0,
        lf_checker_rt::relocated(TAG)
    );
    if r & 0xFF != 0 {
        (r & 0xFFFF_FF00) | 1
    } else {
        r & 0xFFFF_FF00
    }
});
