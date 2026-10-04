// original: 0x00bf4510 emit_header
/// Emit the 0x2a header record, then clear the marker at +2.
export!(thiscall, rw_bf4510(this: *mut u8, a0: u32, a1: u32) -> u32 {
    unsafe {
        let g = *global::<u32>(0x11735A4);
        let ans: u32 = callee_thiscall!(1, u32, this as u32, 0x2a, g, a1, a0, 0);
        *this.add(2) = 0;
        ans
    }
});
