// original: 0x00C2F620 aud_placeable_tracker_state_init

/// Initialize the tracker state addressed by `this`: clear the words at byte
/// offsets `0x00`, `0x04`, `0x08`, `0x0c`, `0x14`, `0x18`, `0x20` and `0x24`,
/// store the exact `1.0f32` pattern at `0x10`, and store `1000.0f32` at `0x1c`.
/// The method returns the same object pointer in EAX and has no stack
/// arguments.
lf_checker_rt::export!(thiscall, rw_00c2f620(this: *mut u8) -> *mut u8 {
    const ONE_BITS: u32 = 0x3f80_0000;
    const THOUSAND_BITS: u32 = 0x447a_0000;
    const ZERO_BITS: u32 = 0;
    unsafe {
        for offset in [0x00usize, 0x04, 0x08, 0x0c, 0x14, 0x18, 0x20, 0x24] {
            this.add(offset).cast::<u32>().write_unaligned(ZERO_BITS);
        }
        this.add(0x10).cast::<u32>().write_unaligned(ONE_BITS);
        this.add(0x1c).cast::<u32>().write_unaligned(THOUSAND_BITS);
    }
    this
});
