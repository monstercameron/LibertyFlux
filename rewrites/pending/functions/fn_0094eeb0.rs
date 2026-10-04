// original: 0x0094eeb0 reset_state_b
/// Clear the second header layout: a word, a half-word and the head word.
export!(thiscall, rw_0094eeb0(obj: *mut u8) -> u32 {
    unsafe {
        *(obj.add(4) as *mut u32) = 0;
        *(obj.add(0x86) as *mut u16) = 0;
        *(obj as *mut u32) = 0;
        0
    }
});
