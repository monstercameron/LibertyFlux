// original: 0x008a8550 audio_vector_weighted_response
/// Weight an audio source by its direction vector against two thresholds.
///
/// Normalizes the input direction (a zero vector keeps a zero scale instead
/// of dividing by zero) and reduces it to one scalar. When the limit at
/// +0x171C is exactly -1.0, or the scalar reaches the limit, the result is
/// 0.0 with 1.0 stored to the optional output. When the scalar is at or
/// above the bound at +0x1718 the result is the gain at +0x1720 with 0.0
/// stored. Between the thresholds the result interpolates by
/// `(limit - t) / (limit - bound)`, times the gain, with one minus that
/// factor stored. NaN inputs follow the unordered-comparison paths.
export!(thiscall, rw_008a8550(this: *const u8, vec: *const f32, out: *mut f32) -> f64 {
    unsafe {
        let x = *vec;
        let y = *vec.add(1);
        let z = *vec.add(2);
        let len2 = x * x + y * y + z * z;
        let inv = if len2 != 0.0 { 1.0 / len2.sqrt() } else { 0.0 };
        let limit = *(this.add(0x171C) as *const f32);
        let nx = x * inv;
        let ny = y * inv;
        let nz = z * inv;
        let t = (nx + ny) * 0.0 + nz;
        if limit != -1.0 {
            if t >= limit {
                if !out.is_null() {
                    *out = 1.0;
                }
                return 0.0;
            }
            let bound = *(this.add(0x1718) as *const f32);
            if !(bound >= t) {
                let gain = *(this.add(0x1720) as *const f32);
                let k = (limit - t) / (limit - bound);
                if !out.is_null() {
                    *out = 1.0 - k;
                }
                return (gain * k) as f64;
            }
            if !out.is_null() {
                *out = 0.0;
            }
            return (*(this.add(0x1720) as *const f32)) as f64;
        }
        if !out.is_null() {
            *out = 1.0;
        }
        0.0
    }
});

