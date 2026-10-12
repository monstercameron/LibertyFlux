// original: 0x00CF1F80 net_session_record_init

/// Initialize the scalar state and selected byte flags of a session record.
///
/// `this` points to a record of at least `0x8c` bytes. The initializer sets
/// its state word to one, clears the dword and subword fields at `+0x10`,
/// `+0x14`, `+0x20`, `+0x7e`, `+0x82`, `+0x84`, `+0x88`, `+0x6e`, `+0x4e`,
/// and `+0x2e`, and returns `this`. Other bytes and words remain untouched.
lf_checker_rt::export!(thiscall, rw_fn_00cf1f80(this: u32) -> u32 {
    const INITIAL_STATE: u32 = 1;
    const ZERO_WORDS: [u32; 3] = [0x10, 0x14, 0x20];
    const ZERO_BYTE_FLAGS: [usize; 4] = [0x88, 0x6e, 0x4e, 0x2e];
    unsafe {
        let record = this as *mut u8;
        record.cast::<u32>().write_unaligned(INITIAL_STATE);
        for offset in ZERO_WORDS {
            record.add(offset as usize).cast::<u32>().write_unaligned(0);
        }
        record.add(0x7e).cast::<u32>().write_unaligned(0);
        record.add(0x82).cast::<u16>().write_unaligned(0);
        record.add(0x84).write(0);
        for offset in ZERO_BYTE_FLAGS {
            record.add(offset).write(0);
        }
    }
    this
});
