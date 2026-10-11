// original: 0x00AC84D0 clear_stream_reset_state

/// Clear the reset bit and its associated counter when the reset bit is set.
///
/// The thiscall object stores flags at `+0x164` and the associated counter at
/// `+0x16c`. Only flag bit 3 controls the operation. If it is clear, the
/// object is left untouched; if it is set, that bit is cleared and the
/// counter is zeroed.
lf_checker_rt::export!(thiscall, rw_00ac84d0(this: u32) -> () {
    const FLAGS_OFFSET: u32 = 0x164;
    const COUNTER_OFFSET: u32 = 0x16c;
    const RESET_BIT: u32 = 1 << 3;

    unsafe {
        let flags = (this.wrapping_add(FLAGS_OFFSET)) as *mut u32;
        let current = flags.read_unaligned();
        if current & RESET_BIT != 0 {
            flags.write_unaligned(current & !RESET_BIT);
            ((this.wrapping_add(COUNTER_OFFSET)) as *mut u32).write_unaligned(0);
        }
    }
});
