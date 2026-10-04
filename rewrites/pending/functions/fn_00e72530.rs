// original: 0x00e72530 guarded_release_15f8bb8
/// Guarded release of the pointer at 0x15f8bb8.
///
/// Frees the pointer through the global release helper (cdecl/1, stubbed)
/// only when the flag word at 0x15f8bbe is nonzero; otherwise does nothing.
export!(cdecl, rw_00e72530() -> () {
    unsafe {
        if *global::<u16>(0x15f8bbe) != 0 {
            callee_cdecl!(1, u32, *global::<u32>(0x15f8bb8));
        }
    }
});
