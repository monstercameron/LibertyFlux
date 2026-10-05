// original: 0x00a3b3f0 vehicle_state_flags (proposed)

/// Build a vehicle state bitmask from the record at `*(obj + 0x80)`.
///
/// Starts at 1. If the linked word at `+0xf50` is non-null, a marker byte of
/// 2 at its `+0xa60` raises the base to 3, otherwise a non-zero byte at its
/// `+0x219` raises it to 9. Then independent bits are ORed in: 8 when
/// `+0x28` has none of bits `0x7c00`, 2 when the byte at `+0x10b8` is 2, and
/// 4 when the byte at `+0xf16` has bit 1. Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a3b3f0(obj: u32) -> u32 {
    unsafe {
        const INNER: u32 = 0x80;
        const LINK: u32 = 0xF50;
        const MARKER: u32 = 0xA60;
        const ALT_MARKER: u32 = 0x219;
        const MODE_MASK: u32 = 0x7C00;
        const MODE_BITS: u32 = 0x28;
        const KIND_BYTE: u32 = 0x10B8;
        const FLAG_BYTE: u32 = 0xF16;
        let rec = core::ptr::read_unaligned((obj + INNER) as *const u32);
        let mut out: u32 = 1;
        let linked = core::ptr::read_unaligned((rec + LINK) as *const u32);
        if linked != 0 {
            if core::ptr::read((linked + MARKER) as *const u8) == 2 {
                out = 3;
            } else if core::ptr::read((linked + ALT_MARKER) as *const u8) != 0 {
                out = 9;
            }
        }
        if core::ptr::read_unaligned((rec + MODE_BITS) as *const u32) & MODE_MASK == 0 {
            out |= 8;
        }
        if core::ptr::read((rec + KIND_BYTE) as *const u8) == 2 {
            out |= 2;
        }
        if core::ptr::read((rec + FLAG_BYTE) as *const u8) & 2 == 0 {
        } else {
            out |= 4;
        }
        out
    }
});
