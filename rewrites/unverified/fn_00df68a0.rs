// original: 0x00DF68A0 UIFileViewer::class_name_hash

/// Return the hash of the UIFileViewer class-name string.
///
/// A cdecl string-hash helper receives the fixed read-only image string and
/// its 32-bit return value is forwarded unchanged.
lf_checker_rt::export!(cdecl, rw_df68a0() -> u32 {
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0xF02114))
});
