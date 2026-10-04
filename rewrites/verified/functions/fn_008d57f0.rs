// original: 0x008D57F0 EmitCenteredBoxPair
/// Emits a fixed box, then a box built from the center pair behind the first
/// argument expanded by the two float radii, and returns the second call's
/// answer.
export!(cdecl, rw_008D57F0(center: u32, rx: f32, ry: f32) -> u32 {
    unsafe {
        let cx = (center as *const f32).read_unaligned();
        let cy = ((center + 4) as *const f32).read_unaligned();
        let mut buf = [
            0xbc23d70au32,
            0xbc23d70a,
            0x3f8147ae,
            0xbc23d70a,
            0x3f8147ae,
            0x3f8147ae,
            0xbc23d70a,
            0x3f8147ae,
        ];
        callee_cdecl!(0, u32, buf.as_mut_ptr() as u32, 0);
        let x0 = cx - rx;
        let x1 = cx + rx;
        let y0 = cy - ry;
        let y1 = cy + ry;
        buf = [
            x0.to_bits(),
            y0.to_bits(),
            x1.to_bits(),
            y0.to_bits(),
            x1.to_bits(),
            y1.to_bits(),
            x0.to_bits(),
            y1.to_bits(),
        ];
        callee_cdecl!(1, u32, buf.as_mut_ptr() as u32, 0)
    }
});
