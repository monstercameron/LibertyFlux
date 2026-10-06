// original: 0x009DBC80 C2dEffect::vf0

/// Scalar deleting destructor: stamp the vtable, free on request.
///
/// `this` arrives in ECX, `flags` is the one stack word (thiscall, callee pops
/// 4 bytes); the return value is `this`. The vtable `VTABLE` is written to
/// `[this]`; when bit 0 of `flags` is set the object is released through
/// operator delete (cdecl, one pointer argument). There is no vector path:
/// bit 1 of `flags` is ignored.
lf_checker_rt::export!(thiscall, rw_009dbc80(this: u32, flags: u32) -> u32 {
    const VTABLE: u32 = 0xe9749c;
    const OP_DELETE: u32 = 1;
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        if flags & 1 != 0 {
            lf_checker_rt::callee_cdecl!(OP_DELETE, u32, this);
        }
    }
    this
});
