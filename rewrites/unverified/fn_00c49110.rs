// original: 0x00c49110 ccamcinematic_maybe_c (proposed)
/// Append entry kind C unless the table already holds entries.
///
/// When the count at `this + COUNT` is not positive, appends via
/// callee 1. Returns 1 in the low byte.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c49110(this: u32) -> u32 {
    const COUNT: u32 = 0x200;
    const APPEND: u32 = 1;
    unsafe {
        if ((this + COUNT) as *const i32).read_unaligned() <= 0 {
            lf_checker_rt::callee_thiscall!(APPEND, u32, this);
        }
    }
    1
});
