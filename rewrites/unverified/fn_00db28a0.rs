// original: 0x00DB28A0 UITexture::get_class_name_hash

/// Return the hash produced by the class-name helper for the read-only
/// `UITexture` name. The function passes the name bytes, including the three
/// observed trailing NUL bytes, to one cdecl helper and returns its EAX value.
lf_checker_rt::export!(thiscall, rw_00db28a0(this: u32) -> u32 {
    static CLASS_NAME: [u8; 12] = *b"UITexture\0\0\0";
    lf_checker_rt::callee_cdecl!(1, u32, CLASS_NAME.as_ptr() as u32)
});
