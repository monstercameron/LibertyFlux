// original: 0x00DC2E20 task_tuning_defaults

/// Store the fixed tuning profile used by the task system.
///
/// The receiver points to a 0x34-byte output record. This initializes eight
/// leading float words through offset 0x1c, three later float words at offsets
/// 0x20, 0x24, and 0x28, another float at 0x2c, and an integer count at 0x30.
lf_checker_rt::export!(thiscall, rw_00DC2E20(output: u32) -> u32 {
    unsafe {
        const FLOAT_DEFAULTS: &[(u32, u32)] = &[
            (0x00, 0x4448_0000),
            (0x04, 0x43c8_0000),
            (0x08, 0x42c8_0000),
            (0x0c, 0x40a0_0000),
            (0x10, 0x4248_0000),
            (0x14, 0x40a0_0000),
            (0x18, 0x4348_0000),
            (0x1c, 0x4248_0000),
            (0x28, 0x42c8_0000),
            (0x24, 0x42c8_0000),
            (0x20, 0x4248_0000),
            (0x2c, 0x42c8_0000),
        ];
        for &(offset, bits) in FLOAT_DEFAULTS {
            (output.wrapping_add(offset) as *mut u32).write_unaligned(bits);
        }
        (output.wrapping_add(0x30) as *mut u32).write_unaligned(10);
    }
    0
});
