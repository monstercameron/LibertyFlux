// original: 0x00CDA750 script_vm_parameter_block_b_init (proposed)

/// Write fixed timing, range, and flag defaults into `this`, including a zero byte at `+0x34`. Unwritten bytes are preserved; thiscall returns `this` in EAX.
lf_checker_rt::export!(thiscall, rw_00cda750(this: u32) -> u32 {
    unsafe {
        const WORD_FIELDS: [(u32, u32); 13] = [
            (0x00, 0x41400000),
            (0x04, 0x3f800000),
            (0x08, 0x3e4ccccd),
            (0x0c, 0xbf0ccccd),
            (0x10, 0x3dcccccd),
            (0x14, 0x3dcccccd),
            (0x18, 0x3dcccccd),
            (0x1c, 0x00000003),
            (0x20, 0x00000008),
            (0x24, 0x000003e8),
            (0x28, 0x000003e8),
            (0x2c, 0x3dcccccd),
            (0x30, 0x00000001),
        ];
        for (offset, value) in WORD_FIELDS {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(value);
        }
        const BYTE_FIELDS: [(u32, u32); 1] = [
            (0x34, 0x00000000),
        ];
        for (offset, value) in BYTE_FIELDS {
            (this.wrapping_add(offset) as *mut u8).write_unaligned(value as u8);
        }
        this
    }
});
