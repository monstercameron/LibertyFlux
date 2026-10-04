// original: 0x00e72b20 guarded_release_16dce68
/// Guarded release of the pointer at 0x16dce68.
///
/// Frees the pointer through the global release helper (cdecl/1, stubbed)
/// only when the flag word at 0x16dce6e is nonzero; otherwise does nothing.
export!(cdecl, rw_00e72b20() -> () {
    unsafe {
        if *global::<u16>(0x16dce6e) != 0 {
            callee_cdecl!(1, u32, *global::<u32>(0x16dce68));
        }
    }
});
