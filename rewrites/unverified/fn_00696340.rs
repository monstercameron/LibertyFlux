// original: 0x00696340 rage::crAnimChannel::eval_indexed

/// Evaluates a channel at an index by virtual dispatch.
///
/// `this` is the channel object; the stack arguments are an integer
/// selector, a float position `t` and an unused word. The position is
/// passed by bits, untouched, to the virtual evaluator in slot 9
/// (callee 1, thiscall: object, selector, `t`-bits, out-slot), which
/// writes the result float through the out-slot: the original's own
/// pushed frame word, primed with `this` (the pointer argument is
/// skipped and the slot snapshotted at the call). The result is loaded
/// with `fld` and returned on the x87 stack, compared bit-exactly
/// through the x87 state check (80-bit extended values, quieted NaN).
///
/// Original: 0x00696340 (thiscall, three stack words, callee pops 12).
lf_checker_rt::export!(thiscall, rw_00696340(this: u32, sel: u32, t_bits: u32, _unused: u32) -> f32 {
    unsafe {
        const EVAL: u32 = 1;
        let mut slot: u32 = this;
        let _ = lf_checker_rt::callee_thiscall!(EVAL, u32, this, sel, t_bits,
            core::ptr::addr_of_mut!(slot) as u32);
        f32::from_bits(slot)
    }
});
