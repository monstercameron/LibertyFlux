// original: 0x00e72660 guarded_release_166541c
/// Guarded release of the pointer at 0x166541c.
///
/// Frees the pointer through the global release helper (cdecl/1, stubbed)
/// only when the flag word at 0x1665422 is nonzero; otherwise does nothing.
export!(cdecl, rw_00e72660() -> () {
    unsafe {
        if *global::<u16>(0x1665422) != 0 {
            callee_cdecl!(1, u32, *global::<u32>(0x166541c));
        }
    }
});
