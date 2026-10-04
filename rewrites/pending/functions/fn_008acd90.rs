// original: 0x008acd90 audio_copy_40_bytes
/// Audio state copy: move 40 bytes from `src+4` to `dst`, if `dst` is set.
///
/// A null destination is a no-op. Returns the destination pointer, matching
/// the value the original leaves in EAX on both paths.
export!(thiscall, rw_008acd90(src: *const u8, dst: *mut u8) -> u32 {
    unsafe {
        if !dst.is_null() {
            core::ptr::copy_nonoverlapping(src.add(4), dst, 40);
        }
        dst as u32
    }
});
