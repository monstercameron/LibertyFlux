// original: 0x00e728d0 guarded_release_16c5380
/// Guarded release of the pointer at 0x16C5380.
///
/// Same shape as [`rw_00e726f0`] with flag word at 0x16C5386.
export!(cdecl, rw_00e728d0() -> () {
    unsafe {
        if *global::<u16>(0x16C5386) != 0 {
            callee_cdecl!(1, u32, *global::<u32>(0x16C5380));
        }
    }
});
