// original: 0x00DECCD0 UIStackLayout::UIStackLayout_2

/// Reset the layout object's most-derived state to the base layout state and
/// then invoke the base teardown routine. The object is addressed through its
/// 32-bit `this` pointer. The inherited vtable word and byte at offset `0xc7`
/// are reset before the intercepted base routine runs. This body has no stack
/// parameters and has no declared return value.
lf_checker_rt::export!(thiscall, rw_00deccd0(this: u32) -> u32 {
    unsafe {
        const VTABLE_WORD: u32 = 0x00;
        const FLAGS_BYTE: u32 = 0xc7;
        const BASE_LAYOUT_VTABLE: u32 = 0x00effcd4;
        (this.wrapping_add(VTABLE_WORD) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(BASE_LAYOUT_VTABLE));
        (this.wrapping_add(FLAGS_BYTE) as *mut u8).write(0);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        0
    }
});
