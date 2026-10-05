// original: 0x00ab6740 stream_maybe_init (proposed)

/// Initialise a streaming endpoint unless disabled.
///
/// When `flag` is zero nothing happens. Otherwise clears the status byte at
/// `+0`, runs the table-setup callee on the sub-object at `+8` with `(0xB,
/// 1)`, clears the status byte again and runs the register callee with the
/// flag as its object and `(0xAB9070, this)` as arguments. No return value.
///
/// Callees: 1 = table setup (thiscall, two words),
/// 2 = endpoint register (thiscall, two words).
///
/// Original: 0x00ab6740 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ab6740(this: u32, flag: u32) -> u32 {
    unsafe {
        const SETUP: u32 = 1;
        const REGISTER: u32 = 2;
        const SUB_OFF: u32 = 8;
        const LANES: u32 = 0x0B;
        const MAGIC: u32 = 0x00AB_9070;
        if flag == 0 {
            return 0;
        }
        (this as *mut u8).write(0);
        lf_checker_rt::callee_thiscall!(SETUP, u32, this.wrapping_add(SUB_OFF), LANES, 1);
        (this as *mut u8).write(0);
        lf_checker_rt::callee_thiscall!(REGISTER, u32, flag, MAGIC, this);
        0
    }
});
