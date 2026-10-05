// original: 0x00B835C0 wrap_B85C20
/// Run the real constructor (callee 1), then return `this`.
///
/// A thin wrapper: the callee's answer is discarded.
///
/// Original: 0x00B835C0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00B835C0(this: u32) -> u32 {
    unsafe {
        lf_checker_rt::callee_thiscall!(1, u32, this);
        this
    }
});
