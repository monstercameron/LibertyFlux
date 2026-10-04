// original: 0x008a4290 rage::audVariableCurveSound::audVariableCurveSound
/// Construct an `audVariableCurveSound`: base init, vtable, curve init.
///
/// Original 0x008A4290 (`thiscall/0`): runs the base constructor, installs
/// the class vtable, runs the curve member's constructor at `this+0xB8`,
/// returns `this`.
export!(thiscall, rw_008a4290(this: u32) -> u32 {    callee_thiscall!(1, u32, this);
    unsafe {
        (this as *mut u32).write_unaligned(relocated(0xe7af9c));
    }
    callee_thiscall!(2, u32, this.wrapping_add(0xb8));
    this
});
