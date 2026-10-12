// original: 0x00D421A0 set_subtask_destination

/// Store the movement destination and associated height, mode and flags into
/// a task object. ECX is the destination object; four stack words contain a
/// mode, a pointer to four source words, a float bit pattern, and flags. The
/// four source words are copied byte-for-byte to +0x40..+0x4c, mode is stored
/// at +0x54, height at +0x50, flags at +0x58, and EAX returns flags. Thiscall
/// consumes all four stack words.
lf_checker_rt::export!(thiscall, rw_D421A0(this: u32, mode: u32, source: u32, height_bits: u32, flags: u32) -> u32 {
    const POSITION_X: u32 = 0x40;
    const HEIGHT: u32 = 0x50;
    const MODE: u32 = 0x54;
    const FLAGS: u32 = 0x58;
    unsafe {
        core::ptr::write_unaligned((this + MODE) as *mut u32, mode);
        for index in 0..4u32 {
            let word = core::ptr::read_unaligned((source + index * 4) as *const u32);
            core::ptr::write_unaligned((this + POSITION_X + index * 4) as *mut u32, word);
        }
        core::ptr::write_unaligned((this + HEIGHT) as *mut u32, height_bits);
        core::ptr::write_unaligned((this + FLAGS) as *mut u32, flags);
    }
    flags
});
