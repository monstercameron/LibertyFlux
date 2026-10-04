// original: 0x0091B3C0 char_byte_map
/// Map one input byte through the fixed game character table.
///
/// Bytes below `0x8E` or above `0xF8` pass through unchanged; bytes
/// `0x8E..=0xCC` shift up by `0x32`; `0xCD` maps to `0xFF`; `0xD3`, `0xF7`
/// and `0xF8` map to `0xB9`, `0xB8` and `0xA8`; every other byte in
/// `0xCE..=0xF6` passes through. Only the low byte of the argument is read.
/// The upper 24 bits of `EAX` are part of the result: mapped bytes return
/// zero-extended, table-identity bytes return the bare byte, and
/// out-of-range bytes keep bits 8..31 of `byte - 0x8E`, exactly as the
/// original's switch leaves them.
export!(cdecl, rw_0091b3c0(arg: u32) -> u32 {
    let c = arg & 0xFF;
    let i = c.wrapping_sub(0x8E);
    if i > 0x6A {
        (i & 0xFFFFFF00) | c
    } else if i <= 62 {
        0xC0 + i
    } else if i == 63 {
        0xFF
    } else if i == 69 {
        0xB9
    } else if i == 105 {
        0xB8
    } else if i == 106 {
        0xA8
    } else {
        c
    }
});
