// original: 0x008F8E80 stream_slot_zero_4b0
/// Zero a streaming slot and set its trailer fields.
///
/// Clears 0x12c dwords at the object, writes a zero dword at `+0x4b0`,
/// the marker word 0x0100 at `+0x4b4` and a zero byte at `+0x4b6`.
/// Thiscall with no stack arguments; returns 0.
export!(thiscall, rw_008f8e80(this: u32) -> u32 {
    unsafe {
        const WORDS: u32 = 0x12c;
        const TRAILER: u32 = 0x4b0;
        for i in 0..WORDS {
            ((this + i * 4) as *mut u32).write_unaligned(0);
        }
        ((this + TRAILER) as *mut u32).write_unaligned(0);
        ((this + TRAILER + 4) as *mut u16).write_unaligned(0x0100);
        ((this + TRAILER + 6) as *mut u8).write(0);
        0
    }
});
