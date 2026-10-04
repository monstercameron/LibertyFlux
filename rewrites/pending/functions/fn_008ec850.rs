// original: 0x008ec850 NativeImpl_GET_SPAWN_COORDINATES_FOR_CAR_NODE
/// Solve a spawn pose for a car node and publish it into the out-slots.
///
/// `this` is the owning table object, `node_id` packs a table index (low
/// word) and a record offset (high word, scaled by 32), `inp` points at
/// three input floats, `out` receives four pose floats and `out2` a single
/// angle float.
///
/// The node record gives three fixed-point coordinates that are scaled and
/// published first. The record's tag nibble then bounds a loop over linked
/// row records: each iteration measures the scaled distance between the
/// node and its partner, normalises it, blends it against the input
/// floats, and keeps the blend only when it beats a running threshold
/// (which starts at a strongly negative constant and then tracks the last
/// accepted blend, so later iterations compare against earlier results).
/// Accepted blends publish three averaged coordinates plus a zero, ask the
/// first callee for a weight, and combine the weight with the direction
/// into the final pose; the second callee observes the call. The angle
/// slot receives the negated x direction converted to degrees.
///
/// Returns `out2` unchanged when the tag nibble is zero, otherwise the
/// tag's low nibble. All floating-point work is single-precision and
/// order-exact: NaN payloads and signed zeroes match the original bit for
/// bit.
export!(thiscall, rw_008ec850(this: u32, node_id: u32, inp: u32, out: u32, out2: u32) -> u32 {
    unsafe {
        let thresh_const = global::<f32>(0x00E833EC).read();
        let s5 = global::<f32>(0x00FE87A4).read();
        let s7 = global::<f32>(0x00FE8720).read();
        let inv_c = global::<f32>(0x00FE88E8).read();
        let half = global::<f32>(0x00FE8830).read();
        let r2d = global::<f32>(0x00E7C2A8).read();
        let sign_bit = global::<u32>(0x00FE8FA0).read();
        let idx = node_id & 0xFFFF;
        let node = (((this
            .wrapping_add(idx.wrapping_mul(4))
            .wrapping_add(0x804)) as *const u32)
            .read())
        .wrapping_add((node_id >> 16).wrapping_shl(5));
        let w14 = |p: u32, off: usize| ((p as *const i16).byte_add(off).read()) as i32 as f32;
        *(out as *mut f32) = w14(node, 0x14) * s5;
        *((out as *mut f32).add(1)) = w14(node, 0x16) * s5;
        *((out as *mut f32).add(2)) = w14(node, 0x18) * s7;
        core::ptr::write(out2 as *mut u32, 0);
        if ((node as *const u8).add(0x1e).read() & 0x0f) == 0 {
            return out2;
        }
        let entry_base = (node as *const i16).byte_add(0x12).read() as i32;
        let mut thresh = thresh_const;
        let mut edi = 0i32;
        loop {
            let rows = (global::<u8>(0x01178384) as u32)
                .wrapping_add(idx.wrapping_mul(4)) as *const u32;
            let rowrec = rows.read().wrapping_add(
                (entry_base.wrapping_add(edi) as u32).wrapping_mul(8),
            );
            let row = (rowrec as *const u32).read();
            let base = ((this
                .wrapping_add((row as u16 as u32).wrapping_mul(4))
                .wrapping_add(0x804)) as *const u32)
                .read();
            if base != 0 {
                let node2 = base.wrapping_add((row >> 16).wrapping_shl(5));
                let a0 = w14(node, 0x14);
                let a1 = w14(node, 0x16);
                let a2 = w14(node, 0x18);
                let b0 = w14(node2, 0x14);
                let b1 = w14(node2, 0x16);
                let b2 = w14(node2, 0x18);
                let mem08a = a2 * s7;
                let t_a0s = a0 * s5;
                let t_b1s = b1 * s5;
                let t_a1s = a1 * s5;
                let t_b0s = b0 * s5;
                let t_b2s = b2 * s7;
                let dx = t_b0s - t_a0s;
                let dy = t_b1s - t_a1s;
                let dz = t_b2s - mem08a;
                let dist = dy * dy + dx * dx + dz * dz;
                let inv = if !(dist <= 0.0) {
                    inv_c / dist.sqrt()
                } else {
                    0.0
                };
                let sz = t_b2s + mem08a;
                let sy = t_b1s + t_a1s;
                let sx = t_b0s + t_a0s;
                let wy = inv * dy;
                let wx = inv * dx;
                let wz = dz * inv;
                let syh = sy * half;
                let szh = sz * half;
                let sxh = sx * half;
                let in0 = (inp as *const f32).read();
                let in1 = (inp as *const f32).add(1).read();
                let in2 = (inp as *const f32).add(2).read();
                let ex = in0 - sxh;
                let ey = in1 - syh;
                let ez = in2 - szh;
                let x6 = ey * ey + ex * ex + ez * ez;
                let x2 = if x6 == 0.0 {
                    0.0
                } else {
                    inv_c / x6.sqrt()
                };
                let blend = ey * x2 * wy + ex * x2 * wx + ez * x2 * wz;
                if blend > thresh {
                    *(out as *mut f32) = sxh;
                    *((out as *mut f32).add(1)) = syh;
                    *((out as *mut f32).add(2)) = szh;
                    thresh = blend;
                    *((out as *mut f32).add(3)) = 0.0;
                    let neg_wx = f32::from_bits(wx.to_bits() ^ sign_bit);
                    // The original reloads ecx with the row address ahead of the first
                    // call each round, but the second call reuses whatever the first
                    // stub left in ecx, which is 0: the stub consumes sequence step
                    // 0 and never restores the entry registers. Mirror that.
                    let w: f32 = callee_thiscall!(0, f32, rowrec);
                    *((out as *mut f32).add(1)) = neg_wx * w + syh;
                    *((out as *mut f32).add(0)) = w * wy + sxh;
                    *((out as *mut f32).add(2)) = w * 0.0 + szh;
                    let _: u32 = callee_thiscall!(1, u32, 0);
                    let back = (neg_wx as f64) as f32;
                    *(out2 as *mut f32) = back * r2d;
                }
            }
            edi = edi.wrapping_add(1);
            let bound = ((node as *const u8).add(0x1e).read() & 0x0f) as i32;
            if edi >= bound {
                break;
            }
        }
        u32::from((node as *const u8).add(0x1e).read() & 0x0f)
    }
});
