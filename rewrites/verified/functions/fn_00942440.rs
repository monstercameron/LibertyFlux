// original: 0x00942440 streaming_ring_push32 (proposed)

/// Append a 32-byte record to the object's ring buffer.
///
/// The object holds a write index at `+0x1000` and a fill count at `+0x1008`,
/// both bounded by 0x80 entries of 32 bytes stored inline from `+0`. When
/// the buffer is full (count 0x80) returns 0 without touching anything.
/// Otherwise advances the index modulo 0x80, copies the 32 bytes from the
/// stack argument into the indexed slot, bumps the count and returns 1.
/// Only the low byte of the return value is set.
///
/// Original: 0x00942440 (thiscall, one stack argument; callee pops 4).
lf_checker_rt::export!(thiscall, rw_00942440(this: u32, src: u32) -> u32 {
    unsafe {
        const NEXT: u32 = 0x1000;
        const COUNT: u32 = 0x1008;
        const CAP: u32 = 0x80;
        const SLOT: u32 = 32;
        const LEN: u32 = 32;
        if ((this + COUNT) as *const u32).read_unaligned() == CAP {
            return 0;
        }
        let advanced = ((this + NEXT) as *const u32)
            .read_unaligned()
            .wrapping_add(1);
        let index = if advanced == CAP { 0 } else { advanced };
        ((this + NEXT) as *mut u32).write_unaligned(index);
        let dst = this + index.wrapping_mul(SLOT);
        let mut i = 0u32;
        while i < LEN {
            let b = ((src + i) as *const u8).read();
            ((dst + i) as *mut u8).write(b);
            i += 1;
        }
        let count = (this + COUNT) as *mut u32;
        count.write_unaligned(count.read_unaligned().wrapping_add(1));
        1
    }
});
