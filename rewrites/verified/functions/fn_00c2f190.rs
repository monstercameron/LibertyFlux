// original: 0x00C2F190 vehicle_dynamics_parameters_reset

/// Reset the stored dynamics factors in the parameter record addressed by
/// `this`. The function writes the four-word factor group at byte offsets
/// `0x14`, `0x18`, `0x1c` and `0x20` to the exact `1.0f32` bit pattern, then
/// writes the mode byte at `0x11` to four. Other bytes in the record are left
/// unchanged. The original is a thiscall method with no stack arguments and
/// no semantic return value.
lf_checker_rt::export!(thiscall, rw_00c2f190(this: *mut u8) -> () {
    const ONE_BITS: u32 = 0x3f80_0000;
    unsafe {
        this.add(0x1c).cast::<u32>().write_unaligned(ONE_BITS);
        this.add(0x14).cast::<u32>().write_unaligned(ONE_BITS);
        this.add(0x20).cast::<u32>().write_unaligned(ONE_BITS);
        this.add(0x18).cast::<u32>().write_unaligned(ONE_BITS);
        this.add(0x11).write(4);
    }
});
