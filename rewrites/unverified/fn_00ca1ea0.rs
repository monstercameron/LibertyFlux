// original: 0x00ca1ea0 subobj_init_chain

/// Reset the event link and initialise the three sub-objects.
///
/// Clears the link word through the swap helper (callee 1, passed 0), then
/// initialises the sub-objects at `+0x110`, `+0x80` and `+0x00` in order
/// (callees 2-4; the last is a tail call whose answer is returned).
///
/// Original: 0x00ca1ea0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ca1ea0(this: u32) -> u32 {
    lf_checker_rt::callee_thiscall!(1, u32, this, 0);
    lf_checker_rt::callee_thiscall!(2, u32, this + 0x110);
    lf_checker_rt::callee_thiscall!(3, u32, this + 0x80);
    lf_checker_rt::callee_thiscall!(4, u32, this)
});
