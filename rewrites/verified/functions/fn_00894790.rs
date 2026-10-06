// original: 0x00894790 fetch_word_or_missing
/// Ask the helper for an item and return its first word, or a missing mark.
///
/// Passes a pointer to a slot holding the incoming word, with the object
/// pointer advanced past its header. When the helper answers null, returns
/// the missing mark 0xFFFF; otherwise returns the half-word the answer
/// points at, zero-extended.
export!(thiscall, rw_00894790(this_: u32, arg: u32) -> u32 {
    unsafe {
        let mut slot = arg;
        let found = callee_thiscall!(
            1,
            u32,
            this_.wrapping_add(0x10),
            &mut slot as *mut u32 as u32
        );
        if found == 0 {
            0xffff
        } else {
            *(found as *const u16) as u32
        }
    }
});
