// original: 0x009053a0 fill_grid_corners
/// Fill eight grid corners: scaled integer products converted to float.
///
/// Centres `x` on half the stored extent, steps the second axis down by `y`
/// plus one, multiplies the axis values by the stored stride and writes the
/// eight corner products as floats. Returns the last `stride * x` product.
export!(cdecl, rw_009053a0(x: u32, y: u32, out: u32) -> u32 {
    unsafe {
        let g1 = *global::<u32>(0x10344e4);
        let s = *global::<u32>(0x10344dc) as i32;
        let g1i = g1 as i32;
        let q = g1i.wrapping_sub(g1i >> 31) >> 1;
        let si = x.wrapping_sub(q as u32);
        let mut di = g1.wrapping_sub(q as u32).wrapping_sub(y);
        let v0 = s.wrapping_mul(si as i32);
        di = di.wrapping_sub(1);
        let v1 = s.wrapping_mul(di as i32);
        let c1 = (si as i32).wrapping_add(1);
        let v2 = c1.wrapping_mul(s);
        let v3 = s.wrapping_mul(di as i32);
        let v4 = c1.wrapping_mul(s);
        let c2 = (di as i32).wrapping_add(1);
        let v5 = c2.wrapping_mul(s);
        let v6 = s.wrapping_mul(si as i32);
        let v7 = c2.wrapping_mul(s);
        let o = out as *mut f32;
        o.write(v0 as f32);
        o.add(1).write(v1 as f32);
        o.add(2).write(v2 as f32);
        o.add(3).write(v3 as f32);
        o.add(4).write(v4 as f32);
        o.add(5).write(v5 as f32);
        o.add(6).write(v6 as f32);
        o.add(7).write(v7 as f32);
        v6 as u32
    }
});
