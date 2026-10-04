// original: 0x00901380 ui_point_from_screen
/// Map a screen-space UI point back to normalised space.
///
/// Each output lane is `(input - base * extent) / (gain * extent)`, the
/// inverse of [`rw_00901310`]. Returns `out`.
export!(cdecl, rw_00901380(out: u32, inp: u32) -> u32 {
    unsafe {
        let h1 = *global::<f32>(0x1190e6c);
        let h2 = *global::<f32>(0x1193c58);
        let g0 = *global::<f32>(0x10344b8);
        let g1 = *global::<f32>(0x10344c0);
        let g2 = *global::<f32>(0x10344bc);
        let g3 = *global::<f32>(0x10344c4);
        let x = (inp as *const f32).read();
        let y = (inp as *const f32).add(1).read();
        let r0 = (x - g0 * h1) / (g1 * h1);
        let r1 = (y - g2 * h2) / (g3 * h2);
        (out as *mut f32).write(r0);
        (out as *mut f32).add(1).write(r1);
        out
    }
});
