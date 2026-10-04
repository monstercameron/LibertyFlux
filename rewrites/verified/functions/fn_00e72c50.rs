// original: 0x00e72c50 guarded_release_1713a9c
/// Guarded release of the pointer at 0x1713a9c.
///
/// Frees the pointer through the global release helper (cdecl/1, stubbed)
/// only when the flag word at 0x1713aa2 is nonzero; otherwise does nothing.
export!(cdecl, rw_00e72c50() -> () {
    unsafe {
        if *global::<u16>(0x1713aa2) != 0 {
            callee_cdecl!(1, u32, *global::<u32>(0x1713a9c));
        }
    }
});
