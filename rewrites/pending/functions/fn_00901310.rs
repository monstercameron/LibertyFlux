// original: 0x00901310 ui_point_to_screen
/// Map a normalised UI point to screen space.
///
/// Each output lane is `gain * extent * input + base * extent` with the two
/// extents and four coefficients coming from globals. Returns `out`.
export!(cdecl, rw_00901310(out: u32, inp: u32) -> u32 {
    unsafe {
        let h1 = *global::<f32>(0x1190e6c);
        let h2 = *global::<f32>(0x1193c58);
        let g0 = *global::<f32>(0x10344b8);
        let g1 = *global::<f32>(0x10344c0);
        let g2 = *global::<f32>(0x10344bc);
        let g3 = *global::<f32>(0x10344c4);
        let x = (inp as *const f32).read();
        let y = (inp as *const f32).add(1).read();
        let a = g1 * h1;
        let b = a * x;
        let c = g0 * h1;
        let r0 = b + c;
        let d = g2 * h2;
        let e = g3 * h2;
        let f = e * y;
        let r1 = f + d;
        (out as *mut f32).write(r0);
        (out as *mut f32).add(1).write(r1);
        out
    }
});
