// original: 0x00C2F1C0 vehicle_dynamics_parameters_set_scalar

/// Store `scalar` at byte offset `0x0c` in the dynamics parameter record,
/// clear the four factor words at offsets `0x14`, `0x18`, `0x1c` and `0x20`,
/// and set the mode byte at offset `0x11` to one. The scalar is moved through
/// its IEEE-754 bits so signed zero and NaN payloads are preserved. The
/// original is a thiscall method with one 32-bit stack argument and no
/// semantic return value.
lf_checker_rt::export!(thiscall, rw_00c2f1c0(this: *mut u8, scalar: f32) -> () {
    const ZERO_BITS: u32 = 0;
    unsafe {
        this.add(0x0c).cast::<u32>().write_unaligned(scalar.to_bits());
        this.add(0x1c).cast::<u32>().write_unaligned(ZERO_BITS);
        this.add(0x14).cast::<u32>().write_unaligned(ZERO_BITS);
        this.add(0x20).cast::<u32>().write_unaligned(ZERO_BITS);
        this.add(0x18).cast::<u32>().write_unaligned(ZERO_BITS);
        this.add(0x11).write(1);
    }
});
