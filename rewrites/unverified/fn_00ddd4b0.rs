// original: 0x00DDD4B0 UITextField::vf2
/// Deleting destructor of the text-field object: reset it, then free it
/// when the caller passes the delete flag (bit 0). Returns `this`.
lf_checker_rt::export!(thiscall, rw_ddd4b0(this: u32, flags: u32) -> u32 {
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    if flags & 1 != 0 {
        let _ = lf_checker_rt::callee_cdecl!(2, u32, this);
    }
    this
});
