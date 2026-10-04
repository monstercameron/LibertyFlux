// original: 0x00e72620 guarded_release_16653fc
/// Guarded release of the pointer at 0x16653fc.
///
/// Frees the pointer through the global release helper (cdecl/1, stubbed)
/// only when the flag word at 0x1665402 is nonzero; otherwise does nothing.
export!(cdecl, rw_00e72620() -> () {
    unsafe {
        if *global::<u16>(0x1665402) != 0 {
            callee_cdecl!(1, u32, *global::<u32>(0x16653fc));
        }
    }
});
