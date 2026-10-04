// original: 0x00984140 audio_find_free_slot
/// Find the first free slot among 16 global audio words.
///
/// Returns the index of the first zero dword in the table at 0x1238858,
/// or -1 when every entry is nonzero.
export!(cdecl, rw_00984140() -> i32 {
    unsafe {
        for i in 0..16u32 {
            if *global::<u32>(0x1238858 + i * 4) == 0 {
                return i as i32;
            }
        }
        -1
    }
});
