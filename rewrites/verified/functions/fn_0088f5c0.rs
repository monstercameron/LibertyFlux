// original: 0x0088f5c0 audio_init_state_block
/// Initializes an 81-byte audio state block with its default pattern:
/// marker bytes and words up front, zeroes through the middle ranges,
/// `0x64` gains and `0xFFFF` sentinels at their slots. Returns `this`.
export!(thiscall, rw_0088f5c0(this_ptr: *mut u8) -> u32 {
    unsafe {
        let w = |off: usize, v: u32| {
            core::ptr::write_unaligned(this_ptr.add(off) as *mut u32, v)
        };
        let h = |off: usize, v: u16| {
            core::ptr::write_unaligned(this_ptr.add(off) as *mut u16, v)
        };
        *this_ptr.add(0x00) = 0xFF;
        w(0x01, 0xFFFFFFFF);
        w(0x05, 0xAAAAAAAA);
        w(0x0B, 0xFFFFFFFF);
        w(0x0F, 0);
        w(0x13, 0);
        h(0x17, 0xFFFF);
        w(0x19, 0);
        h(0x1D, 0);
        w(0x1F, 0);
        w(0x23, 0);
        h(0x27, 0);
        h(0x29, 0xFFFF);
        h(0x2B, 0x64);
        w(0x2D, 0);
        w(0x31, 0);
        h(0x35, 0x64);
        *this_ptr.add(0x38) = 0;
        w(0x39, 0);
        w(0x3D, 0);
        w(0x41, 0);
        w(0x45, 0);
        w(0x49, 0);
        w(0x4D, 0);
        this_ptr as u32
    }
});
