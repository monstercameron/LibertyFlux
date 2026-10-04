// original: 0x00e725a0 guarded_release_1657624
/// Guarded release of the pointer at 0x1657624.
///
/// Frees the pointer through the global release helper (cdecl/1, stubbed)
/// only when the flag word at 0x165762a is nonzero; otherwise does nothing.
export!(cdecl, rw_00e725a0() -> () {
    unsafe {
        if *global::<u16>(0x165762a) != 0 {
            callee_cdecl!(1, u32, *global::<u32>(0x1657624));
        }
    }
});
