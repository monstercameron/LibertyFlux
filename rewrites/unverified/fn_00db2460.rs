// original: 0x00DB2460 UITexture::~UITexture__deleting

/// Reset the texture object through its constructor, then send the native
/// release notification only when bit zero of the deletion flags is set.
/// The notification receives the object pointer as its single stack argument;
/// the method returns the receiver and removes its flags argument.
lf_checker_rt::export!(thiscall, rw_00db2460(this: u32, delete_flags: u32) -> u32 {
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    if delete_flags & 1 != 0 {
        let _ = lf_checker_rt::callee_cdecl!(2, u32, this);
    }
    this
});
