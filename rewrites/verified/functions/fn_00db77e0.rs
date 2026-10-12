// original: 0x00DB77E0 UIMusicClip::UIMusicClip_2

/// Install the music-clip vtable, call its no-argument helper, and forward
/// the receiver to the base method that the original tail-jumps to. Return the
/// base-method result unchanged.
lf_checker_rt::export!(thiscall, rw_00db77e0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ef34f4;
        ((this + 0) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
