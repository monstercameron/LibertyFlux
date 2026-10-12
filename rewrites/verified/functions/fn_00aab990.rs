// original: 0x00AAB990 select_effect_record_field
/// Selects one of two record fields using the key tag, a global mode, and
/// whether the record's optional field is populated.
///
/// The key tag is masked to the 0x3C0 bits. Only an exact 0x80 tag reads the
/// global mode and then the optional word at record offset 0x38. A mode other
/// than 2, or a nonzero optional word when the mode is 2, selects offset 0x34;
/// every other path selects offset 0x30. The two pointers are stack arguments.
#[unsafe(no_mangle)]
pub extern "stdcall" fn rw_aab990(record: u32, key_record: u32) -> u32 {
    const KEY_TAG_OFFSET: usize = 0x28 / 4;
    const OPTIONAL_VALUE_OFFSET: usize = 0x38 / 4;
    const SELECTED_FIELD_OFFSET: usize = 0x34 / 4;
    const DEFAULT_FIELD_OFFSET: usize = 0x30 / 4;
    const KEY_TAG_MASK: u32 = 0x3C0;
    const SELECTED_KEY_TAG: u32 = 0x80;
    const REQUIRED_MODE: u32 = 2;
    const MODE_GLOBAL_VA: u32 = 0x011D6FD4;

    let key_tag = unsafe { ((key_record as *const u32).add(KEY_TAG_OFFSET)).read_unaligned() };
    let item = record as *const u32;
    if key_tag & KEY_TAG_MASK == SELECTED_KEY_TAG {
        let mode = unsafe { lf_checker_rt::global::<u32>(MODE_GLOBAL_VA).read_unaligned() };
        if mode != REQUIRED_MODE
            || unsafe { item.add(OPTIONAL_VALUE_OFFSET).read_unaligned() != 0 }
        {
            return unsafe { item.add(SELECTED_FIELD_OFFSET).read_unaligned() };
        }
    }
    unsafe { item.add(DEFAULT_FIELD_OFFSET).read_unaligned() }
}
