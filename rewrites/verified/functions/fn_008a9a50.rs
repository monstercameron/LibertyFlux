// original: 0x008a9a50 audio_lookup_store
/// Guarded lookup: resolve through the helper unless the key is zero.
///
/// The third argument is the key and the fourth is the output slot: a zero
/// key stores 0 into the slot and returns 0 without calling. Otherwise the
/// helper (stdcall/5, stubbed) receives all five arguments with the output
/// pointer in the fourth position; a zero answer is also stored into the
/// slot. Returns the helper answer, or 0 on the early path.
export!(stdcall, rw_008a9a50(a1: u32, a2: u32, key: u32, out: *mut u32, a5: u32) -> u32 {
    unsafe {
        if key == 0 {
            *out = 0;
            0
        } else {
            let answer = callee_stdcall!(1, u32, a1, a2, key, out as u32, a5);
            if answer == 0 {
                *out = 0;
            }
            answer
        }
    }
});
