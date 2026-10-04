// original: 0x0091d0e0 input_code_lookup (proposed)

/// Map an input event `(kind, code)` to an internal command code.
///
/// `kind` selects the device family: 9 maps codes 0-3, 8 maps codes 0-11,
/// and 6 maps single-bit key masks (1, 2, 4 ... 0x8000, each a power of two)
/// plus the extended code 0x100. Any `kind` outside {6, 8, 9}, and any
/// `code` with no entry, yields `NONE` (0xff).
///
/// The original dispatches through three jump tables held in its own code;
/// this rewrite states the same mapping directly, so it reads no image
/// memory. Cdecl, two stack words, result in `eax`, no calls, no writes.
lf_checker_rt::export!(cdecl, rw_0091d0e0(kind: u32, code: u32) -> u32 {
    const NONE: u32 = 0xff;
    match kind {
        9 => match code {
            0 => 0x113,
            1 => 0x112,
            2 => 0x11b,
            3 => 0x11a,
            _ => NONE,
        },
        8 => match code {
            0 => 0x107,
            1 => 0x106,
            2 => 0x104,
            3 => 0x105,
            4 => 0x11f,
            5 => 0x11d,
            6 => 0x11c,
            7 => 0x11e,
            8 => 0x120,
            9 => 0x122,
            10 => 0x121,
            11 => 0x123,
            _ => NONE,
        },
        6 => match code {
            1 => 0x121,
            2 => 0x123,
            4 => 0x120,
            8 => 0x122,
            16 => 0x11f,
            32 => 0x11d,
            64 => 0x11c,
            128 => 0x11e,
            0x100 => 0x125,
            0x200 => 0x110,
            0x400 => 0x118,
            0x800 => 0x124,
            0x1000 => 0x104,
            0x2000 => 0x107,
            0x4000 => 0x105,
            0x8000 => 0x106,
            _ => NONE,
        },
        _ => NONE,
    }
});
