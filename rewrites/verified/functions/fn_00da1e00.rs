// original: 0x00DA1E00 stream_state_gate
/// Selects one of four flag policies: kind zero always succeeds, kind one requires the object's state byte, kind two requires it to be clear, and kind three or any larger value fails.
lf_checker_rt::export!(cdecl, rw_s675_00da1e00(kind: u32, object: u32) -> u32 {
    if kind == 0 { return 1; }
    if kind > 2 { return 0; }
    let state = unsafe { (object.wrapping_add(0x219) as *const u8).read() };
    ((if kind == 1 { state != 0 } else { state == 0 }) as u8) as u32
});
