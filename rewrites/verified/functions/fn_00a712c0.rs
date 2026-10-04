// original: 0x00a712c0 CTaskComplexPlayerGun::vf18

/// Reset hook of the player-gun task: resets the embedded state block, then
/// parks the task's aim fields.
///
/// Runs the block reset routine (callee 1) with `this + BLOCK` in ecx,
/// writes `-1` to the target handle at `this + TARGET`, zeroes the flag
/// byte at `this + FLAGS` and the word at `this + SPARE`, and returns 0.
///
/// Original: thiscall, one stack word that is never read, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a712c0(this: u32, _arg: u32) -> u32 {
    unsafe {
        const BLOCK: u32 = 0x14;
        const TARGET: u32 = 0x24;
        const FLAGS: u32 = 0x28;
        const SPARE: u32 = 0x2c;
        const NO_TARGET: u32 = 0xffff_ffff;
        const RESET_BLOCK: u32 = 1;

        lf_checker_rt::callee_thiscall!(RESET_BLOCK, u32, this.wrapping_add(BLOCK));
        ((this as *mut u32).wrapping_byte_offset(TARGET as isize)).write_unaligned(NO_TARGET);
        ((this as *mut u8).wrapping_byte_offset(FLAGS as isize)).write(0);
        ((this as *mut u32).wrapping_byte_offset(SPARE as isize)).write_unaligned(0);
        0
    }
});
