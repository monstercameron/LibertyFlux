// original: 0x0097ab00 audio_listener_commit
/// Commit a listener snapshot into an audio listener slot.
///
/// `this` is a 0x29-byte listener record with a state flag at +0x28 and
/// `src` points at a 16-byte snapshot. When the slot is fresh (flag == 0)
/// the snapshot is stored into the pending area at +0x10, the live area at
/// +0x00 is cleared and the flag is set to 1; otherwise the snapshot
/// overwrites the live area directly. Returns `src` unchanged.
export!(thiscall, rw_0097ab00(this: *mut u8, src: *const u8) -> u32 {
    unsafe {
        if *this.add(0x28) == 0 {
            core::ptr::copy_nonoverlapping(src, this.add(0x10), 16);
            core::ptr::write_bytes(this, 0, 16);
            *this.add(0x28) = 1;
        } else {
            core::ptr::copy_nonoverlapping(src, this, 16);
        }
        src as u32
    }
});
