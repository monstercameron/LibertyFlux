// original: 0x00ad8fa0 route_ui_span_pick
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

//
// Test a scaled integer span against a frustum-like predicate, then route
// on the halved sizes: spans dominated by one axis go to one of two
// nine-argument handlers, everything else falls through to a
// twelve-argument dispatcher with precomputed span products. The four
// integer arguments are two corner pairs, the float is a depth carried
// through to every call, and the last three words are forwarded untouched.
// Returns the predicate's zero on a miss, otherwise the answer of whichever
// call ran.
export!(cdecl, rw_00ad8fa0(
    x0: i32, y0: i32, x1: i32, y1: i32, depth: f32, a5: u32, a6: u32, a7: u32,
) -> u32 {
    unsafe {
        let r: f32 = global::<f32>(0x00FE8830).read();
        let sx = x0.wrapping_add(y0);
        let sy = x1.wrapping_add(y1);
        let dx = x1.wrapping_sub(y1);
        let dy = y0.wrapping_sub(x0);
        let mut dxf = (dx as f32) * r;
        let mut dyf = (dy as f32) * r;
        dxf = dxf * dxf;
        dyf = dyf * dyf;
        let dist = (dxf + dyf).sqrt();
        let syf = (sy as f32) * r;
        let sxf = (sx as f32) * r;
        let frustum = global::<u32>(0x017F583C).read();
        let hit = callee_thiscall!(
            1, u32, frustum, sxf.to_bits(), syf.to_bits(), depth.to_bits(),
            dist.to_bits(), 0
        );
        if hit == 0 {
            return 0;
        }
        let (mn, mx) = if x1 >= y1 { (y1, x1) } else { (x1, y1) };
        let half_dy = dy / 2;
        let half_span = mx.wrapping_sub(mn) / 2;
        let flag = global::<u8>(0x0103F510).read();
        let thr = global::<i32>(0x0103F514).read();
        if flag != 0 && half_dy > half_span && half_dy > thr {
            let h = half_dy / 2;
            let c = x0.wrapping_add(h.wrapping_mul(2));
            return callee_cdecl!(
                2, u32, c as u32, x0 as u32, y0 as u32, x1 as u32, y1 as u32,
                depth.to_bits(), a5, a6, a7
            );
        }
        if flag != 0 && !(half_dy > half_span) && half_span > thr {
            let h = half_span / 2;
            let c = mn.wrapping_add(h.wrapping_mul(2));
            return callee_cdecl!(
                3, u32, c as u32, x0 as u32, y0 as u32, x1 as u32, y1 as u32,
                depth.to_bits(), a5, a6, a7
            );
        }
        let prod = half_span.wrapping_mul(half_dy);
        let plus = half_span
            .wrapping_add(1)
            .wrapping_mul(half_dy.wrapping_add(1));
        let doubled = prod.wrapping_add(prod);
        callee_cdecl!(
            4, u32, x0 as u32, y0 as u32, x1 as u32, y1 as u32, depth.to_bits(),
            a5, a6, a7, doubled as u32, plus as u32, half_dy as u32,
            half_span as u32
        )
    }
});
