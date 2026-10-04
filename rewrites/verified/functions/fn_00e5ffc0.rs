// original: 0x00e5ffc0 init_timing_records_and_forward
/// Initialises four timing records, then forwards one fixed callback
/// address to the shared mainloop helper.
///
/// Each record is 0x38 bytes at `0x019f238c + i * 0x38`: two zero words,
/// a marker word, eight zero bytes, another zero word, a packed half-word
/// pair (`0x200` in each half), a flag word, and zero padding. A few pad
/// bytes are never written and keep whatever they held. Returns the
/// helper's answer unchanged.
export!(cdecl, rw_00e5ffc0() -> u32 {
    use core::ptr::write_unaligned;
    const BASE: u32 = 0x019F238C;
    const STRIDE: u32 = 0x38;
    const COUNT: u32 = 4;
    const MARKER: u32 = 0x8080_8080;
    const HALF: u32 = 0x200;
    const FLAGS: u32 = 0x1_0101;
    unsafe {
        let packed = HALF | HALF << 16;
        let mut rec = relocated(BASE);
        let mut i = 0;
        while i < COUNT {
            let w = |off: u32| (rec + off) as *mut u32;
            write_unaligned(w(0x00), 0);
            write_unaligned(w(0x04), 0);
            write_unaligned(w(0x08), MARKER);
            write_unaligned((rec + 0x0C) as *mut u64, 0);
            write_unaligned(w(0x14), 0);
            write_unaligned(w(0x18), packed);
            write_unaligned(w(0x1C), packed);
            write_unaligned(w(0x20), FLAGS);
            write_unaligned((rec + 0x24) as *mut u16, 0);
            write_unaligned(w(0x28), 0);
            write_unaligned((rec + 0x2C) as *mut u8, 0);
            write_unaligned(w(0x2E), 0);
            rec += STRIDE;
            i += 1;
        }
        callee_cdecl!(1, u32, relocated(0x00E6F670))
    }
});
