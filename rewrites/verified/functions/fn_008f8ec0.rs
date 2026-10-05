// original: 0x008F8EC0 stream_entry_reset_and_release
/// Reset a streaming entry, then tail-call its resource release.
///
/// Writes the idle pattern (zero head, 0x0100 marker at `+0x50`,
/// -1 links, zero flag bytes) and tail-calls the release routine on
/// the same object. Thiscall, no stack arguments; returns the tail
/// call's result.
export!(thiscall, rw_008f8ec0(this: u32) -> u32 {
    unsafe {
        ((this) as *mut u32).write_unaligned(0);
        ((this + 0x50) as *mut u16).write_unaligned(0x0100);
        ((this + 4) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((this + 8) as *mut u32).write_unaligned(0);
        for off in [0x0cu32, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24] {
            ((this + off) as *mut u32).write_unaligned(0xFFFFFFFF);
        }
        ((this + 0x28) as *mut u8).write(0);
        ((this + 0x38) as *mut u8).write(0);
        callee_thiscall!(1, u32, this)
    }
});
