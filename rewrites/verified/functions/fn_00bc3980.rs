// original: 0x00BC3980 set_weapon_range_flag_from_task (proposed)

/// Copy the source task's low enable bit into bit 24 of a destination flag
/// word. Other destination bits are preserved. The thiscall entry has three
/// stack words; only the first points at the destination structure. The
/// function returns one in AL and pops all three stack words.
lf_checker_rt::export!(thiscall, rw_00BC3980(this: u32, destination: u32, _unused_1: u32, _unused_2: u32) -> u32 {
    const SOURCE_FLAGS: u32 = 0x14;
    const DESTINATION_FLAGS: u32 = 0x260;
    const WEAPON_RANGE_MASK: u32 = 0x0100_0000;

    unsafe {
        let source_byte = ((this.wrapping_add(SOURCE_FLAGS)) as *const u8).read();
        let destination_word = (destination.wrapping_add(DESTINATION_FLAGS)) as *mut u32;
        let old_flags = destination_word.read_unaligned();
        let new_flag = u32::from(source_byte & 1 != 0) << 24;
        destination_word.write_unaligned((old_flags & !WEAPON_RANGE_MASK) | new_flag);
    }
    1
});
