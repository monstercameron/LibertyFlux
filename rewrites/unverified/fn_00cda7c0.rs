// original: 0x00CDA7C0 script_vm_parameter_block_c_init (proposed)

/// Write fixed parameter defaults into `this`, then clear the halfword at `+0x3c` and byte at `+0x3e`. The gaps stay unchanged; thiscall returns `this` in EAX.
lf_checker_rt::export!(thiscall, rw_00cda7c0(this: u32) -> u32 {
    unsafe {
        const WORD_FIELDS: [(u32, u32); 15] = [
            (0x00, 0x3f800000),
            (0x04, 0x3e4ccccd),
            (0x08, 0x41400000),
            (0x0c, 0x3f800000),
            (0x10, 0x42200000),
            (0x14, 0x3e99999a),
            (0x18, 0x3dcccccd),
            (0x1c, 0x3e99999a),
            (0x20, 0x40e00000),
            (0x24, 0x3fa00000),
            (0x28, 0x3f000000),
            (0x2c, 0x3e4ccccd),
            (0x30, 0x3e800000),
            (0x34, 0x3e19999a),
            (0x38, 0xbd4ccccd),
        ];
        for (offset, value) in WORD_FIELDS {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(value);
        }
        const HALF_FIELDS: [(u32, u32); 1] = [
            (0x3c, 0x00000000),
        ];
        for (offset, value) in HALF_FIELDS {
            (this.wrapping_add(offset) as *mut u16).write_unaligned(value as u16);
        }
        const BYTE_FIELDS: [(u32, u32); 1] = [
            (0x3e, 0x00000000),
        ];
        for (offset, value) in BYTE_FIELDS {
            (this.wrapping_add(offset) as *mut u8).write_unaligned(value as u8);
        }
        this
    }
});
