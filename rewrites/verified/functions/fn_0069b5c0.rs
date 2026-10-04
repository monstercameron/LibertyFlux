// original: 0x0069b5c0 rage::crAnimChannelStaticQuaternion::vf16
/// Static-quaternion uniformity check: copies the first quaternion to the
/// output buffer, then verifies every remaining one matches it within the
/// given tolerance (comparing the largest squared component difference
/// against the squared tolerance, NaN-safe like the original's compare
/// ladder). Returns 1 when all match, 0 on the first outlier.
lf_k2_rt::export!(thiscall, rw_0069b5c0(this: *mut u8, quats: *const u8, count: i32, tol_bits: u32) -> u32 {
    unsafe {
        let out = *((this.add(8)) as *const u32) as *mut u8;
        *(out as *mut u64) = *(quats as *const u64);
        *((out.add(8)) as *mut u64) = *((quats.add(8)) as *const u64);
        if count <= 1 {
            return 1;
        }
        let tol = f32::from_bits(tol_bits);
        let tol2 = tol * tol;
        let b0 = f32::from_bits(*((out.add(0)) as *const u32));
        let b1 = f32::from_bits(*((out.add(4)) as *const u32));
        let b2 = f32::from_bits(*((out.add(8)) as *const u32));
        let b3 = f32::from_bits(*((out.add(12)) as *const u32));
        let mut k = 1i32;
        let mut p = quats.add(16);
        while k < count {
            let dx = b0 - f32::from_bits(*((p.add(0)) as *const u32));
            let dy = b1 - f32::from_bits(*((p.add(4)) as *const u32));
            let dz = b2 - f32::from_bits(*((p.add(8)) as *const u32));
            let dw = b3 - f32::from_bits(*((p.add(12)) as *const u32));
            let dx2 = dx * dx;
            let dy2 = dy * dy;
            let dz2 = dz * dz;
            let dw2 = dw * dw;
            let m1 = if dz2 > dw2 { dz2 } else { dw2 };
            let m0 = if dx2 > dy2 { dx2 } else { dy2 };
            let m = if m0 > m1 { m0 } else { m1 };
            if m > tol2 {
                return 0;
            }
            k += 1;
            p = p.add(16);
        }
        1
    }
});
