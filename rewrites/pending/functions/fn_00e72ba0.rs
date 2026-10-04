// original: 0x00e72ba0 guarded_release_16dd668
/// Guarded release of the pointer at 0x16dd668.
///
/// Frees the pointer through the global release helper (cdecl/1, stubbed)
/// only when the flag word at 0x16dd66e is nonzero; otherwise does nothing.
export!(cdecl, rw_00e72ba0() -> () {
    unsafe {
        if *global::<u16>(0x16dd66e) != 0 {
            callee_cdecl!(1, u32, *global::<u32>(0x16dd668));
        }
    }
});
