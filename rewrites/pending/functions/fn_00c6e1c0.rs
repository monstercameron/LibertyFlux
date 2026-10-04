// original: 0x00c6e1c0 CAnimAssociations::vf0
/// Scalar deleting destructor: run the shared teardown on `this`, then free
/// `this` itself when bit 0 of the flags argument is set. Returns `this`.
export!(thiscall, rw_00c6e1c0(this: u32, flags: u32) -> u32 {
    callee_thiscall!(1, u32, this);
    if (flags & 1) != 0 {
        callee_cdecl!(2, u32, this);
    }
    this
});
