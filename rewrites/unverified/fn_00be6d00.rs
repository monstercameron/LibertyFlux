// original: 0x00BE6D00 sit_task_set_idle_state (proposed)
/// Store the idle-state fields of a seated-task object.
///
/// Writes `a0` to `this+0x34`, `a1` to `this+0x30`, the low byte of `a2` to
/// `this+0x38` and the word 1 to `this+0x39`. Returns eax as the original
/// leaves it: the high three bytes of `a1` with the low byte of `a2` (the
/// last two loads are a dword then a byte). Original is thiscall with three
/// stack words and pops 12 bytes.
lf_checker_rt::export!(thiscall, rw_00be6d00(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    const F_A0: u32 = 0x34;
    const F_A1: u32 = 0x30;
    const F_B: u32 = 0x38;
    const F_W: u32 = 0x39;
    unsafe {
        (this.wrapping_add(F_A0) as *mut u32).write_unaligned(a0);
        (this.wrapping_add(F_A1) as *mut u32).write_unaligned(a1);
        (this.wrapping_add(F_B) as *mut u8).write(a2 as u8);
        (this.wrapping_add(F_W) as *mut u16).write_unaligned(1);
    }
    (a1 & 0xFFFFFF00) | (a2 & 0xFF)
});
