// original: 0x00ABE080 input_ui_dispatch_if_enabled

/// Dispatch a value through a helper when the object's type bits select the
/// enabled case.
///
/// The cdecl argument points to an object. Bits 6 through 9 of its word at
/// offset 0x28 are masked and compared with 0x40. If they differ, the masked
/// value is returned with its low byte cleared. If they match, the word at
/// offset 0x34 is treated as a pointer; null returns zero, otherwise its
/// first word is passed to the helper and the helper's EAX result is returned.
/// The checker scripts the direct helper call and compares the argument.
lf_checker_rt::export!(cdecl, rw_00abe080(object: u32) -> u32 {
    unsafe {
        const FLAGS_OFFSET: u32 = 0x28;
        const VALUE_POINTER_OFFSET: u32 = 0x34;
        const ENABLED_MASK: u32 = 0x3c0;
        const ENABLED_VALUE: u32 = 0x40;

        let type_bits = ((object.wrapping_add(FLAGS_OFFSET) as *const u32).read_unaligned()) & ENABLED_MASK;
        if type_bits != ENABLED_VALUE {
            return type_bits & !0xff;
        }
        let value_pointer = (object.wrapping_add(VALUE_POINTER_OFFSET) as *const u32).read_unaligned();
        if value_pointer == 0 {
            return 0;
        }
        let value = (value_pointer as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(1, u32, value)
    }
});
