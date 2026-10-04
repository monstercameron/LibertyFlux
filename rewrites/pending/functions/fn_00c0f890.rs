// original: 0x00c0f890 UILayoutFrame::copy24_to_0x10
/// Copy 24 bytes from the given source block into fields `0x10..0x28`
/// of this layout frame. The source and destination are distinct objects.
/// Returns the source pointer.
export!(thiscall, rw_00c0f890(this: *mut u8, src: *const u8) -> u32 {
    unsafe {
        core::ptr::copy_nonoverlapping(src, this.add(0x10), 24);
        src as u32
    }
});
