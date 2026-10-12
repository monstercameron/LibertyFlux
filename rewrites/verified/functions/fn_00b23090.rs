// original: 0x00B23090 CPhysical::vf0
/// Forward destruction to the base cleanup helper, conditionally release
/// `this` when the low bit of the flags word is set, then return `this`.
/// Thiscall with one stack argument. Both calls are observed through scripts.
lf_checker_rt::export!(thiscall, rw_00b23090(this: u32, flags: u32) -> u32 {
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        if flags & 1 != 0 {
            let _ = lf_checker_rt::callee_cdecl!(2, u32, this);
        }
        this
    }
});
