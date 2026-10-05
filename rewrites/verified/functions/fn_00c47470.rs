// original: 0x00c47470 ccamscripted_set_flag4 (proposed)
/// Switch the mode tracked by flag bit 2, notifying members.
///
/// The low byte of `arg` is the requested mode. When flag bit 2 of
/// `this + FLAGS` is clear and the mode is 0, returns at once. When the
/// bit is clear and the mode is not 0, enables the member (callee 2)
/// and notifies the object at `this + LISTENER` (callee 3). When the
/// bit is set and the mode is 0, disables the member (callee 1). Then
/// folds the mode into the flag byte: `bit2 = arg & 1` when the mode
/// is not 0, else bit 2 is cleared (the original's shift/xor/mask
/// sequence). No value is returned.
///
/// Original: thiscall, one stack word, callee cleanup (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c47470(this: u32, arg: u32) -> u32 {
    const FLAGS: u32 = 0x264;
    const LISTENER: u32 = 0x12c;
    const MODE_BIT: u8 = 4;
    const DISABLE: u32 = 1;
    const ENABLE: u32 = 2;
    const NOTIFY: u32 = 3;
    unsafe {
        let mode = (arg & 0xff) as u8;
        let flag = ((this + FLAGS) as *const u8).read();
        if flag & MODE_BIT == 0 {
            if mode == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(ENABLE, u32, this);
            let listener = ((this + LISTENER) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(NOTIFY, u32, listener);
        } else if mode == 0 {
            lf_checker_rt::callee_thiscall!(DISABLE, u32, this);
        }
        let cur = ((this + FLAGS) as *const u8).read();
        let mut bl = mode.wrapping_shl(2) ^ cur;
        bl &= MODE_BIT;
        ((this + FLAGS) as *mut u8).write(cur ^ bl);
    }
    0
});
