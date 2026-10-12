// original: 0x00DB4830 UITexture::set_mapping_parameter_and_refresh_extent

/// Store the caller's mapping word, then call the receiver's two no-argument
/// virtual extent methods and save their single-precision x87 results at
/// +0x20C and +0x210. The method pops its one stack word and has no defined
/// return value.
lf_checker_rt::export!(thiscall, rw_00db4830(this: u32, mapping_word: u32) -> u32 {
    unsafe {
        const FIRST_FIELD: u32 = 0x204;
        const FIRST_RESULT: u32 = 0x20c;
        const SECOND_RESULT: u32 = 0x210;
        const FIRST_SLOT: u32 = 0xc8;
        const SECOND_SLOT: u32 = 0xd0;
        ((this + FIRST_FIELD) as *mut u32).write_unaligned(mapping_word);

        let vtable = (this as *const u32).read_unaligned();
        let first_address = (vtable.wrapping_add(FIRST_SLOT) as *const u32).read_unaligned();
        let first_method: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(first_address as usize);
        let first_extent = first_method(this);
        ((this + FIRST_RESULT) as *mut f32).write(first_extent);

        let vtable = (this as *const u32).read_unaligned();
        let second_address = (vtable.wrapping_add(SECOND_SLOT) as *const u32).read_unaligned();
        let second_method: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(second_address as usize);
        let second_extent = second_method(this);
        ((this + SECOND_RESULT) as *mut f32).write(second_extent);
    }
    0
});
