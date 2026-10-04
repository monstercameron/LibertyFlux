// original: 0x00d90e00 audio_cell_fan_area
/// Total area of a cell's triangle fan around its own centroid.
///
/// Resolves the cell's centroid through the centroid callee and each
/// corner through the vertex-fetch callee, then sums Heron's area of every
/// (centroid, previous corner, corner) triangle, closing the fan with the
/// last corner fetched up front. Triangles whose Heron product is at most
/// 1e-4 (degenerate or NaN) contribute nothing. Additions and products
/// that can see two NaNs keep the original's operand order via
/// [`fadd`]/[`fmul`]; the remaining operations have at most one NaN input
/// or are non-commutative, so plain operators match bit-exactly.
export!(thiscall, rw_00d90e00(this: u32, index: u32) -> f32 {
    let entries = unsafe { *((this + 0x6c) as *const u32) };
    let entry = entries.wrapping_add(index.wrapping_mul(40));
    let e0 = unsafe { *(entry as *const u32) };
    let count = (e0 >> 0x15) & 0xf;
    let base = unsafe { *((entry + 4) as *const u32) } & 0x1ffff;
    let verts = unsafe { *((this + 0x60) as *const *const u16) };
    let mut c = [0.0f32; 3];
    callee_thiscall!(1, u32, this, index, c.as_mut_ptr() as u32);
    let mut prev = [0.0f32; 3];
    let v_last = unsafe { *verts.add((base + count).wrapping_sub(1) as usize) } as u32;
    callee_thiscall!(2, u32, this, v_last, prev.as_mut_ptr() as u32);
    let mut acc = 0.0f32;
    if e0 & 0x1e00000 != 0 {
        let mut i = 0u32;
        while i < count {
            let v = unsafe { *verts.add((base + i) as usize) } as u32;
            let mut new = [0.0f32; 3];
            callee_thiscall!(3, u32, this, v, new.as_mut_ptr() as u32);
            let t1 = prev[0] - c[0];
            let t2 = prev[1] - c[1];
            let t3 = prev[2] - c[2];
            let a = fadd(fadd(t2 * t2, t1 * t1), t3 * t3).sqrt();
            let d4 = new[1] - prev[1];
            let d1 = new[0] - prev[0];
            let d5 = new[2] - prev[2];
            let b = fadd(fadd(d4 * d4, d1 * d1), d5 * d5).sqrt();
            let d3 = c[1] - new[1];
            let d2 = c[0] - new[0];
            let d6 = c[2] - new[2];
            let cc = fadd(fadd(d3 * d3, d2 * d2), d6 * d6).sqrt();
            let p = fadd(fadd(a, cc), b);
            let u = fmul(fmul(fmul(p - cc * 2.0, p), p - a * 2.0), p - b * 2.0);
            if u > 1e-4 {
                acc = fadd(0.25 * u.sqrt(), acc);
            }
            prev = new;
            i += 1;
        }
    }
    acc
});
