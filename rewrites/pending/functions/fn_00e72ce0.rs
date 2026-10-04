// original: 0x00e72ce0 guarded_release_171c104
/// Guarded release of the pointer at 0x171c104.
///
/// Frees the pointer through the global release helper (cdecl/1, stubbed)
/// only when the flag word at 0x171c10a is nonzero; otherwise does nothing.
export!(cdecl, rw_00e72ce0() -> () {
    unsafe {
        if *global::<u16>(0x171c10a) != 0 {
            callee_cdecl!(1, u32, *global::<u32>(0x171c104));
        }
    }
});
