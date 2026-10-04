// original: 0x00968230 read_timer_state
/// Copy timer state bytes to two optional out-pointers; return the flag.
///
/// When `out0` is non-null and the flag byte at `this + 0x3027` is
/// nonzero, the byte at `this + 0x3025` is stored to `*out0`. When `out1`
/// is non-null, the dword at `this + 0x3028` is stored to `*out1`
/// unconditionally. Returns the flag byte in `al` (upper `eax` is a
/// leftover, hence the `al` return channel).
///
/// Original: 0x00968230 (thiscall, two stack words).

export!(thiscall, rw_00968230(this: u32, out0: u32, out1: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x3027;
        if out0 != 0 && (this.wrapping_add(FLAG) as *const u8).read() != 0 {
            (out0 as *mut u8).write((this.wrapping_add(0x3025) as *const u8).read());
        }
        if out1 != 0 {
            (out1 as *mut u32)
                .write_unaligned((this.wrapping_add(0x3028) as *const u32).read_unaligned());
        }
        (this.wrapping_add(FLAG) as *const u8).read() as u32
    }
});
