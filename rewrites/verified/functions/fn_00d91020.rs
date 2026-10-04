// original: 0x00d91020 audio_position_dispatch
/// Dispatch a positional audio update for the entity to one of two paths.
///
/// Refreshes entities flagged at bit 2 of the byte at offset 0x50, then
/// builds the adjusted position (x and y carried over, z lowered by the
/// caller's height) and hands it with the source pointer and tag to the
/// near path when the word at offset 0x70 is set, else to the far path.
/// Returns the chosen path's answer.
lf_rs89_rt::export!(thiscall, rw_00d91020(this: *mut u8, src: u32, tag: u32, height_bits: u32) -> u32 {
    unsafe {
        if *(this.wrapping_add(0x50) as *const u8) & 4 != 0 {
            lf_rs89_rt::callee_thiscall!(1, u32, this as u32);
        }
        let x = *(src as *const f32);
        let y = *(src.wrapping_add(4) as *const f32);
        let z = *(src.wrapping_add(8) as *const f32) - f32::from_bits(height_bits);
        let mut pos = [x, y, z];
        if *(this.wrapping_add(0x70) as *const u32) != 0 {
            lf_rs89_rt::callee_thiscall!(2, u32, this as u32, src, &mut pos as *mut f32 as u32, tag, 1)
        } else {
            lf_rs89_rt::callee_thiscall!(3, u32, this as u32, src, &mut pos as *mut f32 as u32, tag, 1)
        }
    }
});
