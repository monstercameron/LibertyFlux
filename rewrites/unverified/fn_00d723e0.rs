// original: 0x00D723E0 remove_replay_marker_array_entry

/// Passes the array segment pointer and its next word to array_segment_copy_down, returning the helper's result. The wrapper has one stdcall pointer argument.
lf_checker_rt::export!(stdcall, rw_00d723e0(segment: u32) -> u32 {
    unsafe {
        lf_checker_rt::callee_stdcall!(1, u32, segment, segment.wrapping_add(4))
    }
});
