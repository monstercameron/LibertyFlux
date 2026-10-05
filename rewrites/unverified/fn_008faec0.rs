// original: 0x008FAEC0 stream_lane0_idle_check
/// Test whether streaming lane 0 is idle.
///
/// Calls the lane-active check for lane 0 and returns 1 when it
/// reports inactive, else 0. Cdecl, no arguments.
export!(cdecl, rw_008faec0() -> u32 {
    unsafe {
        let r: u32 = callee_stdcall!(1, u32, 0);
        if (r as u8) != 0 { 0 } else { 1 }
    }
});
