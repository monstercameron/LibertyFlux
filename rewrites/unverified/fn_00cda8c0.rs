// original: 0x00CDA8C0 script_vm_parameter_block_d_init (proposed)

/// Write the fixed parameter block at `this+0x00..0x64`. All listed values are stored as raw 32-bit words; thiscall returns `this` in EAX.
lf_checker_rt::export!(thiscall, rw_00cda8c0(this: u32) -> u32 {
    unsafe {
        const WORD_FIELDS: [(u32, u32); 26] = [
            (0x00, 0x3ecccccd),
            (0x04, 0x41500000),
            (0x08, 0x3f8ccccd),
            (0x0c, 0x3f333333),
            (0x10, 0x3f800000),
            (0x14, 0x3fb33333),
            (0x18, 0x41300000),
            (0x1c, 0x3f000000),
            (0x20, 0x3f800000),
            (0x24, 0x40000000),
            (0x28, 0x41600000),
            (0x2c, 0x3f800000),
            (0x30, 0x3f4ccccd),
            (0x34, 0x3f800000),
            (0x38, 0x3fb33333),
            (0x3c, 0x41200000),
            (0x40, 0x3f000000),
            (0x44, 0x3f000000),
            (0x48, 0x40000000),
            (0x4c, 0x41200000),
            (0x50, 0x3f000000),
            (0x54, 0x3f99999a),
            (0x58, 0x3f800000),
            (0x5c, 0x3fa00000),
            (0x60, 0x000005dc),
            (0x64, 0x430c0000),
        ];
        for (offset, value) in WORD_FIELDS {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(value);
        }
        this
    }
});
