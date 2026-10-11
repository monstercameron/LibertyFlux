// original: 0x00CDA9F0 script_vm_parameter_block_f_init (proposed)

/// Write the fixed parameter block at the recorded word offsets, plus a zero byte at `+0x98`. Padding and the gap from `+0x7c` through `+0x8b` remain unchanged; thiscall returns `this` in EAX.
lf_checker_rt::export!(thiscall, rw_00cda9f0(this: u32) -> u32 {
    unsafe {
        const WORD_FIELDS: [(u32, u32); 31] = [
            (0x00, 0x41580000),
            (0x04, 0x3f800000),
            (0x08, 0x3e800000),
            (0x0c, 0x00000006),
            (0x10, 0x40400000),
            (0x14, 0x40400000),
            (0x18, 0x00000190),
            (0x1c, 0x000003e8),
            (0x2c, 0x3dcccccd),
            (0x30, 0x3e4ccccd),
            (0x34, 0x3f800000),
            (0x38, 0x40000000),
            (0x3c, 0x40800000),
            (0x40, 0x3f800000),
            (0x44, 0x40000000),
            (0x48, 0x3f000000),
            (0x4c, 0x3f800000),
            (0x50, 0x3e800000),
            (0x54, 0x3f4ccccd),
            (0x58, 0x3f19999a),
            (0x5c, 0x3fd9999a),
            (0x60, 0x40400000),
            (0x64, 0x3f800000),
            (0x68, 0x42480000),
            (0x6c, 0x42b40000),
            (0x70, 0x00000000),
            (0x74, 0x40800000),
            (0x78, 0x3f000000),
            (0x8c, 0x3fe00000),
            (0x90, 0x000003e8),
            (0x94, 0x00000000),
        ];
        for (offset, value) in WORD_FIELDS {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(value);
        }
        const BYTE_FIELDS: [(u32, u32); 1] = [
            (0x98, 0x00000000),
        ];
        for (offset, value) in BYTE_FIELDS {
            (this.wrapping_add(offset) as *mut u8).write_unaligned(value as u8);
        }
        this
    }
});
