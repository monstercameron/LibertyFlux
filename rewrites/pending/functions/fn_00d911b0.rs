// original: 0x00d911b0 audio_child_forward_gate
/// Forward a query to the child object linked at `this + 0x70`, if any.
///
/// When the link is null the result is 0. Otherwise the linked object and
/// the caller's argument are passed to the shared box-query routine and its
/// answer is returned unchanged.
lf_rs89_rt::export!(thiscall, rw_00d911b0(this: u32, arg: u32) -> u32 {
    unsafe {
        let child = *(this.wrapping_add(0x70) as *const u32);
        if child == 0 {
            0
        } else {
            lf_rs89_rt::callee_thiscall!(1, u32, this, child, arg)
        }
    }
});
