// original: 0x008aad90 audio_handle_open_store
/// Open a handle through the helper and store it into the output slot.
///
/// Calls the helper (stdcall/1, stubbed) with the key, writes the answer
/// into `*out`, and returns the answer with its low byte replaced by the
/// nonzero flag (matching exit EAX).
export!(stdcall, rw_008aad90(key: u32, out: *mut u32) -> u32 {
    unsafe {
        let answer = callee_stdcall!(1, u32, key);
        *out = answer;
        (answer & 0xFFFF_FF00) | ((answer != 0) as u32)
    }
});
