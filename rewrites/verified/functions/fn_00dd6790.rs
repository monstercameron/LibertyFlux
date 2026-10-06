// original: 0x00dd6790 UIBasicClip::vf2

/// Destroy the clip, freeing it when the flag asks.
///
/// `this` is the clip object and `flag` is one stack word. Destructor
/// helper 1 is called with `this` in ECX; then, only when the low bit of
/// `flag` is set, helper 2 (cdecl, one word) is called with `this`. The
/// object pointer itself is returned in EAX either way.
///
/// Original: thiscall, one stack word, callee pops it (the callee pops 4 bytes), address
/// result in EAX.
lf_checker_rt::export!(thiscall, rw_00dd6790(this: u32, flag: u32) -> u32 {
    const DTOR: u32 = 1;
    const FREE: u32 = 2;
    unsafe {
        lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if flag & 1 != 0 {
            lf_checker_rt::callee_cdecl!(FREE, u32, this);
        }
        this
    }
});
