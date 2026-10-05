// original: 0x008FB770 stream_mgr_has_pending
/// Test whether the streaming manager has a pending request.
///
/// Returns whether the word at `+0x9a8` is nonzero. Thiscall with no
/// stack arguments, boolean in al.
export!(thiscall, rw_008fb770(this: u32) -> u32 {
    unsafe {
        const PENDING: u32 = 0x9a8;
        let w = ((this + PENDING) as *const u32).read_unaligned();
        (w != 0) as u32
    }
});
