// original: 0x00e727f0 guarded_release_16c5388
/// Guarded release of the pointer at 0x16C5388.
///
/// Same shape as [`rw_00e726f0`] with flag word at 0x16C538E.
export!(cdecl, rw_00e727f0() -> () {
    unsafe {
        if *global::<u16>(0x16C538E) != 0 {
            callee_cdecl!(1, u32, *global::<u32>(0x16C5388));
        }
    }
});
