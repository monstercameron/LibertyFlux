// original: 0x00C62D40 anim_player_clear_flag_bit

/// If the animation player's entry count at offset `0x44` is nonzero, clears
/// bit 0 in its flags word at offset `0x46`. Otherwise it leaves the flags
/// untouched. This is a thiscall method with no stack arguments and a void
/// result; the accumulator value is not part of its interface.
lf_checker_rt::export!(thiscall, rw_00c62d40(this: u32) -> () {
    const ENTRY_COUNT: u32 = 0x44;
    const FLAGS: u32 = 0x46;
    const CLEAR_LOW_BIT: u16 = 0xFFFE;

    unsafe {
        let count = (this.wrapping_add(ENTRY_COUNT) as *const u16).read_unaligned();
        if count != 0 {
            let flags = this.wrapping_add(FLAGS) as *mut u16;
            flags.write_unaligned(flags.read_unaligned() & CLEAR_LOW_BIT);
        }
    }
});
