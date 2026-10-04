// original: 0x009526b0 stamped_base_or_next
/// Ask the stamp helper for its value and return the stored base, or the
/// base plus one when the stamp matches the expected word.
export!(cdecl, rw_009526b0() -> u32 {
    unsafe {
        let stamp: u32 = callee_cdecl!(1, u32,);
        let expected = *global::<u32>(0x011f702c);
        let base = *global::<u32>(0x011f70c4);
        if stamp == expected {
            base.wrapping_add(1)
        } else {
            base
        }
    }
});
