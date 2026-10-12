// original: 0x00c602f0 CTaskSimpleBeHit::vf17

/// Returns true immediately when the task's state byte at offset `0x18` is
/// nonzero. Otherwise, when the word at offset `0x24` is zero, it calls the
/// task's animation starter with the forwarded ped pointer and then returns
/// false. The byte is tested for zero, not masked to one bit. This thiscall
/// method has one stack pointer argument and returns its boolean in `AL`.
lf_checker_rt::export!(thiscall, rw_00c602f0(this: u32, ped: u32) -> u8 {
    const STATE: u32 = 0x18;
    const ANIMATION_HANDLE: u32 = 0x24;

    unsafe {
        if (this.wrapping_add(STATE) as *const u8).read() != 0 {
            return 1;
        }
        if (this.wrapping_add(ANIMATION_HANDLE) as *const u32).read_unaligned() == 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, this, ped);
        }
    }
    0
});
