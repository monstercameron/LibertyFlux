// original: 0x00dfb950 stack_align_tail_00dfb950
/// Alignment helper: round the entry value up to the slot stride, hand off.
///
/// Takes its input value in the entry accumulator on the original side;
/// the rewrite reads the same value from its stack word (both run one
/// lockstep cycle, so equality of the value is verified and only the
/// transport differs). Adds the low nibble of the slot address minus the
/// value, saturating to all set bits on overflow, and transfers control
/// to the shared successor with the result in the accumulator, returning
/// whatever that call answers. The accumulator result is compared through
/// the register log.
export!(cdecl, rw_00dfb950(proxy: u32) -> u32 {
    let slot = &proxy as *const u32 as u32;
    let k = slot.wrapping_sub(proxy) & 0xF;
    let (res, carry) = proxy.overflowing_add(k);
    let aligned = if carry { 0xFFFFFFFF } else { res };
    callee_cdecl!(1, u32, aligned)
});
