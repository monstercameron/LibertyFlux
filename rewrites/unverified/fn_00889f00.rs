// original: 0x00889F00 audio_voice_param_block_init (proposed)

/// Reset a voice parameter block to its default state.
///
/// `this` points to a 0x70-byte block. Writes 1.0 to the two gain words at
/// `+0x00` and `+0x1c`, the count 20000 at `+0x10`, 0xFFFF at `+0x14`, the
/// marker bytes 0x00/0xFF at `+0x6c`/`+0x6e`, sets byte `+0x18` to 1 and sets
/// bit 0 of byte `+0x19` (read-modify-write: the old value's other bits are
/// kept), and zeroes every other word from `+0x04` to `+0x6b` (the tail range
/// is zeroed twice, once with 8-byte stores and once with a 3-iteration loop
/// over 4 words). Returns `this`.
///
/// Original: 0x00889F00 (thiscall, `this` in ecx, no stack words).
lf_checker_rt::export!(thiscall, rw_00889f00(this: u32) -> u32 {
    unsafe {
        const GAIN_ONE: u32 = 0x3f80_0000;
        const DEFAULT_COUNT: u32 = 0x4e20;
        const FULL_MASK: u16 = 0xffff;
        const MARK_WORD: u16 = 0xff00;
        const MARK_BYTE: u8 = 0xff;
        const ENABLE_BIT: u8 = 0x01;

        let base = this as *mut u8;
        // First gain word and its padding.
        (base as *mut u32).write_unaligned(GAIN_ONE);
        (base.add(0x04) as *mut u32).write_unaligned(0);
        (base.add(0x08) as *mut u32).write_unaligned(0);
        (base.add(0x0c) as *mut u32).write_unaligned(0);
        // Count and flags.
        (base.add(0x10) as *mut u32).write_unaligned(DEFAULT_COUNT);
        (base.add(0x14) as *mut u16).write_unaligned(FULL_MASK);
        (base.add(0x16) as *mut u16).write_unaligned(0);
        (base.add(0x1a) as *mut u16).write_unaligned(0);
        // Second gain word and following words.
        (base.add(0x1c) as *mut u32).write_unaligned(GAIN_ONE);
        (base.add(0x20) as *mut u32).write_unaligned(0);
        (base.add(0x24) as *mut u32).write_unaligned(0);
        (base.add(0x28) as *mut u32).write_unaligned(0);
        (base.add(0x2c) as *mut u32).write_unaligned(0);
        (base.add(0x30) as *mut u32).write_unaligned(0);
        (base.add(0x34) as *mut u32).write_unaligned(0);
        (base.add(0x38) as *mut u32).write_unaligned(0);
        // Tail range 0x3c..0x6c zeroed with 8-byte stores ...
        for off in [0x3cu32, 0x44, 0x4c, 0x54, 0x5c, 0x64] {
            (base.add(off as usize) as *mut u64).write_unaligned(0);
        }
        // ... and again word by word (3 iterations over 4 words).
        let mut w = (base.add(0x48)) as *mut u32;
        for _ in 0..3u32 {
            w.sub(3).write_unaligned(0);
            w.write_unaligned(0);
            w.add(3).write_unaligned(0);
            w.add(6).write_unaligned(0);
            w = w.add(1);
        }
        // Marker and enable bytes.
        (base.add(0x6c) as *mut u16).write_unaligned(MARK_WORD);
        base.add(0x6e).write(MARK_BYTE);
        base.add(0x18).write(1);
        let keep = base.add(0x19).read();
        base.add(0x19).write(keep | ENABLE_BIT);
        this
    }
});
