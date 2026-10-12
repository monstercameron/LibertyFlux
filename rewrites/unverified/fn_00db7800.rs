// original: 0x00DB7800 UIMusicViewer::UIMusicViewer_2

/// Install the music-viewer vtable, call its no-argument helper, and forward
/// the receiver to the base method that the original tail-jumps to. Return the
/// base-method result unchanged.
lf_checker_rt::export!(thiscall, rw_00db7800(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ef36f4;
        ((this + 0) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
