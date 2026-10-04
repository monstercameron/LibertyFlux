// original: 0x0069b370 rage::crAnimChannelStaticFloat::vf14
/// Check that `count` strided floats starting at `base` all lie within `tol`
/// of the first: stores `*base` at `this+8`, then for each following element
/// at `base + (stride * 4 + 4) * k` compares the squared difference against
/// the squared tolerance, returning 0 in AL on the first outlier and 1 when
/// all match (or when `count <= 1`, compared signed).
export!(thiscall, rw_0069b370(this: u32, base: u32, count: u32, stride: u32, tol: f32) -> u32 {
    unsafe {
        let first = *(base as *const f32);
        *((this + 8) as *mut f32) = first;
        let step = stride.wrapping_mul(4).wrapping_add(4);
        if (count as i32) <= 1 {
            return 1;
        }
        let limit = tol * tol;
        let mut i: i32 = 1;
        let mut ptr = base.wrapping_add(step);
        while i < count as i32 {
            let diff = first - *(ptr as *const f32);
            if diff * diff > limit {
                return 0;
            }
            i += 1;
            ptr = ptr.wrapping_add(step);
        }
        1
    }
});
