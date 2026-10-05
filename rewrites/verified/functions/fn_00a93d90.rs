// original: 0x00a93d90 stream_maybe_advance_clock (proposed)

/// Advance the stream clock when the running flag is set.
///
/// Shifts the state word at `[this]` right by one and tests its low bit
/// (original bit 1). When clear, the shifted word is returned and nothing
/// else happens. When set, the float at `this+0x04` is passed to the
/// advance callee and its answer is returned.
///
/// Original: thiscall, no stack arguments. One callee (thiscall, 1 arg).
lf_checker_rt::export!(thiscall, rw_00a93d90(this: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x00;
        const STAMP: u32 = 0x04;
        const RUNNING_BIT: u32 = 0;
        const ADVANCE: u32 = 0;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let shifted = rd32(this.wrapping_add(STATE)) >> 1;
        if shifted & 1 << RUNNING_BIT == 0 {
            return shifted;
        }
        let stamp = rd32(this.wrapping_add(STAMP));
        lf_checker_rt::callee_thiscall!(ADVANCE, u32, this, stamp)
    }
});
