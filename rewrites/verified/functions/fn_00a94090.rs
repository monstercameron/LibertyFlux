// original: 0x00a94090 stream_clear_flag_bits_all (proposed)

/// Clear a set of flag bits in every entry of a streaming entry array.
///
/// `this+0x00` holds the entry array pointer and `this+0x04` the signed
/// entry count; each entry is 0x18 bytes with a flag word at `+0x0e`. Every
/// entry's flag word is ANDed with the bitwise complement of the argument
/// (only the low 16 bits of the complement are applied, as a word AND).
/// A count of zero or less does nothing.
///
/// Returns the entry array pointer (0 when the count check fails and entry
/// eax is 0, as the contract pins it). Thiscall: object in ecx, one stack
/// word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a94090(this: u32, mask: u32) -> u32 {
    unsafe {
        const ARRAY: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const ENTRY_STRIDE: u32 = 0x18;
        const FLAGS_OFF: u32 = 0x0e;
        let count = ((this + COUNT) as *const i32).read_unaligned();
        if count <= 0 {
            return 0;
        }
        let base = ((this + ARRAY) as *const u32).read_unaligned();
        let keep = !(mask as u16);
        let mut i = 0i32;
        while i < count {
            let flags = (base + (i as u32) * ENTRY_STRIDE + FLAGS_OFF) as *mut u16;
            flags.write_unaligned(flags.read_unaligned() & keep);
            i += 1;
        }
        base
    }
});
