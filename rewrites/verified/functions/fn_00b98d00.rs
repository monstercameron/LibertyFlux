// original: 0x00b98d00 NativeImpl_IS_GAME_KEYBOARD_KEY_PRESSED

/// Returns whether a keyboard key is held down, as a normalized flag.
///
/// Same shape as `rw_00b98cd0`: calls the input method `CALLEE` on the
/// global input object `OBJ` with (`name`, 1, `KIND`). The result keeps the
/// callee's upper three bytes and sets bit 0 when the callee's low byte is
/// nonzero. `bl` is zeroed before the call and read back after it, relying
/// on the callee preserving `ebx`.
///
/// The pushed object pointer and string address are absolute immediates
/// with relocation entries, so both sides push the relocated addresses and
/// the call log compares them as image-relative offsets.
///
/// Original: 0x00B98D00 (cdecl, one stack word, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b98d00(name: u32) -> u32 {
    const OBJ: u32 = 0x0118D110;
    const KIND: u32 = 0x00EB5CE0;
    const CALLEE: u32 = 1;
    let r: u32 = lf_checker_rt::callee_thiscall!(
        CALLEE,
        u32,
        lf_checker_rt::relocated(OBJ),
        name,
        1,
        lf_checker_rt::relocated(KIND)
    );
    if r & 0xFF != 0 {
        (r & 0xFFFF_FF00) | 1
    } else {
        r & 0xFFFF_FF00
    }
});
