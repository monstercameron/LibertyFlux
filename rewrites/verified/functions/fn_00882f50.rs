// original: 0x00882f50 stream_req_init (proposed)
/// Initialise a streaming request object.
///
/// Zeroes the header words (`+0x00`, `+0x08`, `+0x0c`, `+0x10`), the flag byte
/// at `+0x04`, the two pointer-sized fields at `+0x24`/`+0x28`, and the flag
/// byte at `+0x32`, and writes the constant marker `0x0101` to the half-word
/// at `+0x30`. Returns the object pointer unchanged.
///
/// Original: thiscall, no stack arguments, returns `this` in `eax`.
lf_checker_rt::export!(thiscall, rw_00882f50(this: u32) -> u32 {
    unsafe {
        const MARKER: u16 = 0x0101;
        let base = this as *mut u8;
        // Header: dword, flag byte, then three more dwords.
        (base as *mut u32).write_unaligned(0);
        base.add(0x04).write(0);
        (base.add(0x08) as *mut u32).write_unaligned(0);
        (base.add(0x0c) as *mut u32).write_unaligned(0);
        (base.add(0x10) as *mut u32).write_unaligned(0);
        // Tail fields and the marker half-word.
        (base.add(0x24) as *mut u32).write_unaligned(0);
        (base.add(0x28) as *mut u32).write_unaligned(0);
        (base.add(0x30) as *mut u16).write_unaligned(MARKER);
        base.add(0x32).write(0);
        this
    }
});
