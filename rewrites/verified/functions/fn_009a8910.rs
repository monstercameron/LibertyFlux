// original: 0x009a8910 byte_pair_gate
/// Test two flag bytes on the object: true unless the first is clear or the second set.
///
/// Reads the byte at `this+0x08`; when it is zero the answer is false.
/// Otherwise reads the byte at `this+0x0b`; the answer is true only when
/// that byte is zero. Thiscall, no stack arguments, byte result in AL.
export!(thiscall, rw_009A8910(this: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 0x08;
        const SECOND: u32 = 0x0b;
        if ((this + FIRST) as *const u8).read() == 0 {
            return 0;
        }
        (((this + SECOND) as *const u8).read() == 0) as u32
    }
});
