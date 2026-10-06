// original: 0x00DDD4B0 UITextField::vf2 deleting destructor
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Destroy the field: run the destructor, then free the object when bit 0
/// of `flags` (low byte) is set. Returns `this`.
/// Original: thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00DDD4B0(this: u32, flags: u32) -> u32 {
    unsafe {
        const DTOR: u32 = 1;
        const OP_DELETE: u32 = 2;
        lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if (flags & 1) != 0 {
            lf_checker_rt::callee_cdecl!(OP_DELETE, u32, this);
        }
        this
    }
});
