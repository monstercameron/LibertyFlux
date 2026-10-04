// original: 0x008d4e50 Streaming_GetIdFromName
/// Name-to-id lookup: forwards the name pointer and a zero seed to the hash
/// worker and returns its answer.
export!(cdecl, rw_008d4e50(name: u32) -> u32 {
    unsafe { callee_cdecl!(1, u32, name, 0) }
});
