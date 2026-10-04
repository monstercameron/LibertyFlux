// original: 0x0094eea0 reset_state_a
/// Clear the first header layout: two half-words and the head word.
export!(thiscall, rw_0094eea0(obj: *mut u8) -> u32 {
    unsafe {
        *(obj.add(4) as *mut u16) = 0;
        *(obj.add(0x84) as *mut u16) = 0;
        *(obj as *mut u32) = 0;
        0
    }
});
