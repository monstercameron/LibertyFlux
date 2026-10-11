// original: 0x00CDA980 script_vm_parameter_block_e_init (proposed)

/// Write five fixed integer defaults at offsets `+0x00..+0x10`; remaining bytes are preserved. The zero-argument thiscall returns `this` in EAX.
lf_checker_rt::export!(thiscall, rw_00cda980(this: u32) -> u32 {
    unsafe {
        const WORD_FIELDS: [(u32, u32); 5] = [
            (0x00, 0x00000064),
            (0x04, 0x00000320),
            (0x08, 0x000000c8),
            (0x0c, 0x000002bc),
            (0x10, 0x000003e8),
        ];
        for (offset, value) in WORD_FIELDS {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(value);
        }
        this
    }
});
