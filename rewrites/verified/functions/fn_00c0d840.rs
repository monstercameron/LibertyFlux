// original: 0x00C0D840 UILayoutFrame::vf41
/// Report whether the dword state field at `this + 0xa8` is nonzero. The
/// method takes `this` in ECX and returns a zero-extended boolean in EAX.
lf_checker_rt::export!(thiscall, rw_c0d840(this: u32) -> u32 {
    unsafe {
        const STATE_ID: u32 = 0x0a8;
        let state_id = (this.wrapping_add(STATE_ID) as *const u32).read_unaligned();
        u32::from(state_id != 0)
    }
});
