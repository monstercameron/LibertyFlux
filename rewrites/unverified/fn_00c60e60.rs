// original: 0x00C60E60 CTaskSimpleSetCharIgnoreWeaponRangeFlag::process_ped

/// Copies the task's active byte into bit 24 of the supplied character flag
/// word. The low bit at task offset `0x14` selects the flag state; all other
/// bits in the destination word are preserved. This is a 32-bit thiscall
/// method with one stack pointer argument and an `AL` result of true.
lf_checker_rt::export!(thiscall, rw_00c60e60(this: u32, character: u32) -> u8 {
    const TASK_ACTIVE: u32 = 0x14;
    const CHARACTER_FLAGS: u32 = 0x260;
    const IGNORE_WEAPON_RANGE: u32 = 0x0100_0000;

    unsafe {
        let active = ((this.wrapping_add(TASK_ACTIVE) as *const u8).read() & 1) as u32;
        let flags = character.wrapping_add(CHARACTER_FLAGS) as *mut u32;
        let old = flags.read_unaligned();
        let desired = active << 24;
        flags.write_unaligned((old & !IGNORE_WEAPON_RANGE) | desired);
    }
    1
});
