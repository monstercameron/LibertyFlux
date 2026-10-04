// original: 0x00e726f0 guarded_release_166d9b8
/// Guarded release of the pointer at 0x166D9B8.
///
/// Frees the pointer through the global release helper (cdecl/1, stubbed)
/// only when the flag word at 0x166D9BE is nonzero; otherwise does nothing.
export!(cdecl, rw_00e726f0() -> () {
    unsafe {
        if *global::<u16>(0x166D9BE) != 0 {
            callee_cdecl!(1, u32, *global::<u32>(0x166D9B8));
        }
    }
});
