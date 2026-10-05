// original: 0x009a9c60 indexed_byte_fetch
/// Fetch one tag byte from the row table at `this+0x543`.
///
/// Returns the zero-extended byte at `this + row + 0x543`. Thiscall,
/// one stack word, dword result holding the byte.
export!(thiscall, rw_009A9C60(this: u32, row: u32) -> u32 {
    unsafe {
        const TAG_TABLE: u32 = 0x543;
        ((this + row + TAG_TABLE) as *const u8).read() as u32
    }
});
