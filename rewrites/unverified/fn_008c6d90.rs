// original: 0x008C6D90 blob21_zero
/// Reset a small streaming record to zero, keeping its identity slot.
///
/// Writes zero to the four dwords at `this`, the flag byte at `this + 0x10`
/// and the dword at `this + 0x14`; the three bytes at `0x11..0x14` are left
/// untouched. Returns `this`. Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_008c6d90(this: u32) -> u32 {
    unsafe {
        for off in [0u32, 4, 8, 12] {
            ((this + off) as *mut u32).write_unaligned(0);
        }
        ((this + 0x10) as *mut u8).write(0);
        ((this + 0x14) as *mut u32).write_unaligned(0);
        this
    }
});
