// original: 0x00d8e290 audio_flag_set_by_state
/// Set bit 0 of the status word at `this + 4` when the audio state allows.
///
/// Polls the shared state reader twice: a zero word on the first poll ends
/// the call, otherwise the second poll's handle is resolved and bit 0 is
/// set only when the resolved state is at least 2, compared as a signed
/// 32-bit value. Returns the last answer seen (zero on the early path).
lf_rs89_rt::export!(thiscall, rw_00d8e290(this: *mut u8) -> u32 {
    let this_addr = this as u32;
    let first: u32 = lf_rs89_rt::callee_thiscall!(1, u32, this_addr);
    if first == 0 {
        return 0;
    }
    let handle: u32 = lf_rs89_rt::callee_thiscall!(1, u32, this_addr);
    let state: u32 = lf_rs89_rt::callee_thiscall!(2, u32, handle);
    if (state as i32) >= 2 {
        unsafe {
            let status = this.wrapping_add(4) as *mut u32;
            *status |= 1;
        }
    }
    state
});
