// original: 0x008f5300 key_event_byte_map (proposed)

/// Map one input byte to the byte stored for it, honouring a bypass flag.
///
/// `code` is the input byte (low byte of the first word), `out` receives the
/// single result byte, and `flag` (low byte of the third word) selects the
/// mapping: when it is zero the code is stored unchanged, otherwise codes
/// whose SIGNED value plus 0x58 falls in 0..=0x57 (an UNSIGNED bound check)
/// are translated through a fixed table and every other code is stored
/// unchanged. The table covers exactly the codes 0xA8, 0xB8-0xB9, 0xC0-0xDF
/// and 0xE0-0xFF; all remaining codes pass through on both paths.
///
/// The original leaves the `out` pointer in EAX on the mapping path and the
/// code byte in AL (upper bytes untouched) on the bypass path; all six
/// callers ignore the return value, so the contract compares no return
/// channel. Stdcall, three words, callee cleans up.
#[inline(always)]
fn map_idx(idx: u32, code_b: u8) -> u8 {
    match idx {
            0x00 => 0xF8,
            0x10 => 0xF7,
            0x11 => 0xD3,
            0x18 => 0x8E,
            0x19 => 0x8F,
            0x1A => 0x90,
            0x1B => 0x91,
            0x1C => 0x92,
            0x1D => 0x93,
            0x1E => 0x94,
            0x1F => 0x95,
            0x20 => 0x96,
            0x21 => 0x97,
            0x22 => 0x98,
            0x23 => 0x99,
            0x24 => 0x9A,
            0x25 => 0x9B,
            0x26 => 0x9C,
            0x27 => 0x9D,
            0x28 => 0x9E,
            0x29 => 0x9F,
            0x2A => 0xA0,
            0x2B => 0xA1,
            0x2C => 0xA2,
            0x2D => 0xA3,
            0x2E => 0xA4,
            0x2F => 0xA5,
            0x30 => 0xA6,
            0x31 => 0xA7,
            0x32 => 0xA8,
            0x33 => 0xA9,
            0x34 => 0xAA,
            0x35 => 0xAB,
            0x36 => 0xAC,
            0x37 => 0xAD,
            0x38 => 0xAE,
            0x39 => 0xAF,
            0x3A => 0xB0,
            0x3B => 0xB1,
            0x3C => 0xB2,
            0x3D => 0xB3,
            0x3E => 0xB4,
            0x3F => 0xB5,
            0x40 => 0xB6,
            0x41 => 0xB7,
            0x42 => 0xB8,
            0x43 => 0xB9,
            0x44 => 0xBA,
            0x45 => 0xBB,
            0x46 => 0xBC,
            0x47 => 0xBD,
            0x48 => 0xBE,
            0x49 => 0xBF,
            0x4A => 0xC0,
            0x4B => 0xC1,
            0x4C => 0xC2,
            0x4D => 0xC3,
            0x4E => 0xC4,
            0x4F => 0xC5,
            0x50 => 0xC6,
            0x51 => 0xC7,
            0x52 => 0xC8,
            0x53 => 0xC9,
            0x54 => 0xCA,
            0x55 => 0xCB,
            0x56 => 0xCC,
            0x57 => 0xCD,
        _ => code_b,
    }
}

lf_checker_rt::export!(stdcall, rw_008f5300(code: u32, out: u32, flag: u32) -> u32 {
    unsafe {
        let code_b = code as u8;
        let v = if (flag as u8) == 0 {
            code_b
        } else {
            // Signed byte plus 0x58, compared UNSIGNED against 0x57.
            let idx = (code_b as i8 as i32).wrapping_add(0x58) as u32;
            if idx <= 0x57 {
                map_idx(idx, code_b)
            } else {
                code_b
            }
        };
        (out as *mut u8).write(v);
        out
    }
});
