// original: 0x00ad2b60 audio_mode_preset_apply

/// Apply one of six audio mode presets by programming parameter slots.
///
/// `mode` selects the preset: -1, 0, 1, 2, 3 or 4 (as signed 32-bit). Each
/// preset is a fixed ordered list of (slot, value) pairs written through
/// the two-argument cdecl setter callee (id 1): the first call argument is
/// the slot id, the second the value. Modes -1, 0, 1 and 2 program fifteen
/// slots; modes 3 and 4 program fourteen, sharing one three-call tail that
/// writes slots 0x20, 0x1e and 0x1f (the value for 0x20 differs: 3 under
/// mode 3, 0 under mode 4). Any other mode programs nothing.
///
/// One entry is data-dependent: under mode 1, slot 0x13 is programmed with
/// the selector itself (here always 1). The function has no other inputs:
/// it reads no registers, no heap and no globals, and all other values
/// are immediates. It returns the setter's last answer, or the selector
/// unchanged when nothing was programmed (the original falls through to a
/// bare `ret`, leaving the input in eax).
///
/// Original: 0x00ad2b60 (cdecl, one stack word, 85 direct call sites to a
/// single callee, caller-cleaned).
lf_checker_rt::export!(cdecl, rw_00ad2b60(mode: u32) -> u32 {
    const SET_PARAM: u32 = 1;
    /// Table marker meaning "program the selector itself" (mode 1, slot 0x13).
    const VALUE_IS_MODE: u32 = u32::MAX;
    const PRESET_M1: [(u32, u32); 15] = [
        (0x13, 0x00),
        (0x1a, 0xff),
        (0x19, 0xff),
        (0x18, 0x00),
        (0x17, 0x01),
        (0x16, 0x00),
        (0x14, 0x00),
        (0x15, 0x00),
        (0x1d, 0x00),
        (0x21, 0x01),
        (0x20, 0x00),
        (0x1e, 0x00),
        (0x1f, 0x00),
        (0x0f, 0x0f),
        (0x08, 0x01),
    ];
    const PRESET_P0: [(u32, u32); 15] = [
        (0x13, 0x01),
        (0x1a, 0x00),
        (0x19, 0xff),
        (0x18, 0x00),
        (0x17, 0x07),
        (0x16, 0x00),
        (0x14, 0x00),
        (0x15, 0x00),
        (0x1d, 0x01),
        (0x21, 0x07),
        (0x20, 0x00),
        (0x1e, 0x00),
        (0x1f, 0x00),
        (0x0f, 0x0f),
        (0x08, 0x01),
    ];
    const PRESET_P1: [(u32, u32); 15] = [
        (0x13, VALUE_IS_MODE),
        (0x1a, 0xff),
        (0x19, 0xff),
        (0x18, 0x00),
        (0x17, 0x01),
        (0x16, 0x00),
        (0x14, 0x00),
        (0x15, 0x06),
        (0x1d, 0x01),
        (0x21, 0x01),
        (0x20, 0x00),
        (0x1e, 0x00),
        (0x1f, 0x05),
        (0x0f, 0x00),
        (0x08, 0x00),
    ];
    const PRESET_P2: [(u32, u32); 15] = [
        (0x13, 0x01),
        (0x1a, 0xff),
        (0x19, 0xff),
        (0x18, 0x01),
        (0x17, 0x04),
        (0x16, 0x00),
        (0x14, 0x00),
        (0x15, 0x06),
        (0x1d, 0x01),
        (0x21, 0x04),
        (0x20, 0x06),
        (0x1e, 0x00),
        (0x1f, 0x00),
        (0x0f, 0x00),
        (0x08, 0x00),
    ];
    const PRESET_P3: [(u32, u32); 14] = [
        (0x0f, 0x00),
        (0x13, 0x01),
        (0x1a, 0xff),
        (0x19, 0xff),
        (0x18, 0x00),
        (0x17, 0x01),
        (0x16, 0x04),
        (0x14, 0x00),
        (0x15, 0x00),
        (0x1d, 0x01),
        (0x21, 0x01),
        (0x20, 0x03),
        (0x1e, 0x00),
        (0x1f, 0x00),
    ];
    const PRESET_P4: [(u32, u32); 14] = [
        (0x0f, 0x00),
        (0x13, 0x01),
        (0x1a, 0x00),
        (0x19, 0xff),
        (0x18, 0x00),
        (0x17, 0x07),
        (0x16, 0x00),
        (0x14, 0x00),
        (0x15, 0x00),
        (0x1d, 0x01),
        (0x21, 0x07),
        (0x20, 0x00),
        (0x1e, 0x00),
        (0x1f, 0x00),
    ];
    let table: &[(u32, u32)] = match mode as i32 {
        -1 => &PRESET_M1,
        0 => &PRESET_P0,
        1 => &PRESET_P1,
        2 => &PRESET_P2,
        3 => &PRESET_P3,
        4 => &PRESET_P4,
        _ => &[],
    };
    let mut out = mode;
    for &(slot, value) in table {
        let v = if value == VALUE_IS_MODE { mode } else { value };
        out = lf_checker_rt::callee_cdecl!(SET_PARAM, u32, slot, v);
    }
    out
});
