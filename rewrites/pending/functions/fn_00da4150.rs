// original: 0x00da4150 first_of_dword_list_or_null
// s10f06: first dword of a list, or null when it holds no whole dword (thiscall/0).
//
// The list is a begin/end byte pair; only a length with a full dword in it
// (length & ~3 != 0) yields the first element.
export!(thiscall, rw_s10f06(this: *const u8) -> u32 {
    unsafe {
        let begin = *(this.add(0x24) as *const u32);
        let end = *(this.add(0x28) as *const u32);
        if end.wrapping_sub(begin) & 0xFFFF_FFFC != 0 {
            *(begin as *const u32)
        } else {
            0
        }
    }
});
