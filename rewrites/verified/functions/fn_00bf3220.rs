// original: 0x00bf3220 tag_and_store_word
/// Tag `this` and store one word.
///
/// Writes the tag byte `0x21` at `this+0` and `value` at `this+4`. Returns
/// `value`.
export!(thiscall, rw_bf3220(this_obj: u32, value: u32) -> u32 {
    unsafe {
        *(this_obj as *mut u8) = 0x21;
        *((this_obj + 4) as *mut u32) = value;
        value
    }
});
