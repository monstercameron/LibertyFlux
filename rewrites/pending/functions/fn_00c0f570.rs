// original: 0x00c0f570 UILayoutFrame::vf39
/// Store a float into field `0xb0`; when the tag at field `0xb4` differs
/// from the incoming tag, update it and clear the word at field `0xb8`.
/// Returns the incoming tag.
export!(thiscall, rw_00c0f570(this: *mut u8, value: f32, tag: u32) -> u32 {
    unsafe {
        *(this.add(0xb0) as *mut f32) = value;
        let slot = this.add(0xb4) as *mut u32;
        if *slot != tag {
            *slot = tag;
            *(this.add(0xb8) as *mut u32) = 0;
        }
        tag
    }
});
