// original: 0x00d2a6f0 task_field_init (proposed)
/// Initialise a small task field block: byte at `+0` to 0xff, three dwords at
/// `+4` to zero, three bytes at `+0x10` to zero. Returns `this`.
///
/// Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00d2a6f0(this: u32) -> u32 {
    unsafe {
        const MARK: u8 = 0xff;
        const DW0: u32 = 0x04;
        const B0: u32 = 0x10;
        (this as *mut u8).write(MARK);
        for i in 0..3u32 {
            ((this + DW0 + i * 4) as *mut u32).write_unaligned(0);
            ((this + B0 + i) as *mut u8).write(0);
        }
        this
    }
});
