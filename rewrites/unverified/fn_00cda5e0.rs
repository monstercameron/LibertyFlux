// original: 0x00CDA5E0 script_vm_parameter_block_a_init (proposed)

/// Write the fixed parameter words and one halfword into `this`. Gaps are preserved; the zero-argument thiscall returns `this` in EAX.
lf_checker_rt::export!(thiscall, rw_00cda5e0(this: u32) -> u32 {
    unsafe {
        const WORD_FIELDS: [(u32, u32); 18] = [
            (0x00, 0x40400000),
            (0x04, 0x3f333333),
            (0x08, 0x3e19999a),
            (0x0c, 0x3ecccccd),
            (0x10, 0x42200000),
            (0x14, 0x40000000),
            (0x18, 0x3fc00000),
            (0x1c, 0x3f000000),
            (0x20, 0x3fe00000),
            (0x24, 0x00000000),
            (0x28, 0x00000320),
            (0x2c, 0x000000c8),
            (0x30, 0x000002bc),
            (0x34, 0x00000000),
            (0x38, 0x3e800000),
            (0x3c, 0x3f400000),
            (0x40, 0x3f733333),
            (0x44, 0x3f4ccccd),
        ];
        for (offset, value) in WORD_FIELDS {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(value);
        }
        const HALF_FIELDS: [(u32, u32); 1] = [
            (0x50, 0x00000100),
        ];
        for (offset, value) in HALF_FIELDS {
            (this.wrapping_add(offset) as *mut u16).write_unaligned(value as u16);
        }
        this
    }
});
