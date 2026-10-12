// original: 0x00DE3EB0 UIListHeader::class_name_hash

/// Return the UIListHeader class-name hash through the shared string-hash
/// helper. The name pointer is an image reference, so relocation keeps the
/// rewrite tied to the executable's read-only class-name string.
lf_checker_rt::export!(cdecl, rw_00DE3EB0() -> u32 {
    const CLASS_NAME_TEXT: u32 = 0x00EFDF00;
    let name = lf_checker_rt::relocated(CLASS_NAME_TEXT);
    lf_checker_rt::callee_cdecl!(1, u32, name)
});
