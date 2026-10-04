// original: 0x00c0f8f0 UILayoutFrame::vf33
/// Copy 24 bytes from the given source block into fields `0xa4..0xbc`
/// of this layout frame. The source and destination are distinct objects.
/// Returns the source pointer.
export!(thiscall, rw_00c0f8f0(this: *mut u8, src: *const u8) -> u32 {
    unsafe {
        core::ptr::copy_nonoverlapping(src, this.add(0xa4), 24);
        src as u32
    }
});
