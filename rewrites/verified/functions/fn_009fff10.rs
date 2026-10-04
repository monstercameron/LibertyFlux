// original: 0x009FFF10 frag_named_slot_add (proposed)

/// Copy a name string into slot `index` of a fixed-stride table in `obj`.
///
/// `obj` holds a dword count at `+0x2ec`; each slot is `0x2c` bytes. The
/// NUL-terminated string at `name` is copied byte by byte (including the
/// terminator) to slot `index` at offset 8, the dwords `v0` and `v1` are
/// stored at slot offsets 0 and 4, the count is incremented, and the slot
/// index used (the old count) is returned.
///
/// Original: 0x009FFF10 (thiscall, `obj` in `ecx`, three stack words).
lf_checker_rt::export!(thiscall, rw_009FFF10(obj: u32, name: u32, v0: u32, v1: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x2EC;
        const STRIDE: u32 = 0x2C;
        const NAME_OFF: u32 = 8;
        let idx = ((obj + COUNT_OFF) as *const u32).read_unaligned();
        let slot = obj + idx.wrapping_mul(STRIDE);
        let mut s = name;
        let mut d = slot + NAME_OFF;
        loop {
            let b = (s as *const u8).read();
            (d as *mut u8).write(b);
            s += 1;
            d += 1;
            if b == 0 {
                break;
            }
        }
        (slot as *mut u32).write_unaligned(v0);
        ((slot + 4) as *mut u32).write_unaligned(v1);
        ((obj + COUNT_OFF) as *mut u32).write_unaligned(idx + 1);
        idx
    }
});
