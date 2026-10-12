// original: 0x00D92520 audio_params_copy_if_present

/// Calls the per-object audio helper only when the object pointer is non-null.
/// The object is passed in ECX and the second input is forwarded as one stack
/// word. The wrapper has no semantic return value, and a null object skips the
/// call without reading either pointed-to value.
lf_checker_rt::export!(cdecl, rw_00d92520(object: u32, value: u32) -> () {
    if object != 0 {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, object, value);
    }
});
