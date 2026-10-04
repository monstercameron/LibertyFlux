// original: 0x00adaf60 grid_seg_cast
/// Cast a segment through a grid of height entries, reporting the first hit.
///
/// Takes two 3-float endpoints and a 4-float output slot. Both endpoints are
/// folded into per-component minima and maxima, scaled into integer cell
/// coordinates, and every covered cell is visited in order. Ordinary cells
/// look up a table word: one tag selects a height entry directly, another
/// walks a chain of entries. Each entry defines a horizontal plane from the
/// shared grid table; when the endpoints lie strictly on opposite sides, the
/// crossing point is interpolated and must fall inside the entry's index
/// bounds. Out-of-range cells take a separate path against a single level
/// value and re-check the crossed cell. On a hit the point and a zero fourth
/// slot are stored and 1 is returned, else 0. Truncation to integers matches
/// the hardware convert instruction exactly, including its out-of-range
/// result.
export!(cdecl, rw_00adaf60(a: u32, b: u32, out: u32) -> u32 {
    fn cvt(x: f32) -> i32 {
        if x.is_nan() || x >= 2_147_483_648.0 || x < -2_147_483_648.0 {
            i32::MIN
        } else {
            x as i32
        }
    }
    unsafe fn plane_hit(
        entry: u32,
        ax: f32,
        ay: f32,
        az: f32,
        bx: f32,
        by: f32,
        bz: f32,
        lo_z: f32,
        hi_z: f32,
        out: u32,
    ) -> bool {
        unsafe {
            const TABLE: u32 = 0x0158E860;
            const ABS_MASK: u32 = 0x00FE8F80;
            let table = relocated(TABLE);
            let row = |idx: i32| table.wrapping_add((idx.wrapping_mul(8)) as u32);
            let ia = ((entry.wrapping_add(4)) as *const i16).read_unaligned() as i32;
            let plane = ((row(ia).wrapping_add(4)) as *const f32).read_unaligned();
            let da = az - plane;
            let db = bz - plane;
            if !(0.0 > db * da) {
                return false;
            }
            let mask = (relocated(ABS_MASK) as *const u32).read_unaligned();
            let t = f32::from_bits(da.to_bits() & mask) / (hi_z - lo_z);
            let px = (bx - ax) * t + ax;
            let py = (by - ay) * t + ay;
            let pz = (bz - az) * t + az;
            ((out.wrapping_add(0x0c)) as *mut f32).write_unaligned(0.0);
            (out as *mut f32).write_unaligned(px);
            ((out.wrapping_add(4)) as *mut f32).write_unaligned(py);
            ((out.wrapping_add(8)) as *mut f32).write_unaligned(pz);
            let lo_x = (((row(ia)) as *const i16).read_unaligned() as i32) as f32;
            if !(px >= lo_x) {
                return false;
            }
            let ib = ((entry.wrapping_add(6)) as *const i16).read_unaligned() as i32;
            let hi_x = (((row(ib)) as *const i16).read_unaligned() as i32) as f32;
            if !(hi_x >= px) {
                return false;
            }
            let lo_y = (((row(ia).wrapping_add(2)) as *const i16).read_unaligned() as i32) as f32;
            if !(py >= lo_y) {
                return false;
            }
            let ic = ((entry.wrapping_add(8)) as *const i16).read_unaligned() as i32;
            let hi_y = (((row(ic).wrapping_add(2)) as *const i16).read_unaligned() as i32) as f32;
            hi_y >= py
        }
    }
    unsafe {
        const K1_CELL: u32 = 0x00FE86D0;
        const K2_CELL: u32 = 0x00FE8AE0;
        const ABS_MASK: u32 = 0x00FE8F80;
        const T1: u32 = 0x0154E358;
        const T2: u32 = 0x0154E478;
        const ENT: u32 = 0x01550EB0;
        const W0: u32 = 0x0154EC4C;

        let ax = (a as *const f32).read_unaligned();
        let ay = ((a.wrapping_add(4)) as *const f32).read_unaligned();
        let az = ((a.wrapping_add(8)) as *const f32).read_unaligned();
        let bx = (b as *const f32).read_unaligned();
        let by = ((b.wrapping_add(4)) as *const f32).read_unaligned();
        let bz = ((b.wrapping_add(8)) as *const f32).read_unaligned();
        let lo_x = if bx > ax { ax } else { bx };
        let lo_y = if by > ay { ay } else { by };
        let lo_z = if bz > az { az } else { bz };
        let hi_x = if ax > bx { ax } else { bx };
        let hi_y = if ay > by { ay } else { by };
        let hi_z = if az > bz { az } else { bz };
        let k1 = (relocated(K1_CELL) as *const f32).read_unaligned();
        let k2 = (relocated(K2_CELL) as *const f32).read_unaligned();
        let lo_cx = cvt(lo_x * k1 + k2);
        let hi_cx = cvt(hi_x * k1 + k2);
        let lo_cy = cvt(lo_y * k1 + k2);
        let hi_cy = cvt(hi_y * k1 + k2);
        if lo_cx > hi_cx {
            return 0;
        }
        let t1 = relocated(T1);
        let t2 = relocated(T2);
        let ent = relocated(ENT);
        let mut gx = lo_cx;
        let mut base = gx.wrapping_mul(12);
        loop {
            if !(lo_cy > hi_cy) {
                let mut gy = lo_cy;
                loop {
                    if (gx as u32) > 11 || (gy as u32) > 11 {
                        let w0 = (relocated(W0) as *const f32).read_unaligned();
                        if w0 > lo_z && hi_z > w0 {
                            let mask =
                                (relocated(ABS_MASK) as *const u32).read_unaligned();
                            let t =
                                f32::from_bits((az - w0).to_bits() & mask) / (hi_z - lo_z);
                            let px = (bx - ax) * t + ax;
                            let py = (by - ay) * t + ay;
                            let pz = (bz - az) * t + az;
                            (out as *mut f32).write_unaligned(px);
                            ((out.wrapping_add(8)) as *mut f32).write_unaligned(pz);
                            ((out.wrapping_add(0x0c)) as *mut f32).write_unaligned(0.0);
                            ((out.wrapping_add(4)) as *mut f32).write_unaligned(py);
                            if gx == cvt(px * k1 + k2) && gy == cvt(py * k1 + k2) {
                                return 1;
                            }
                        }
                    } else {
                        let idx = base.wrapping_add(gy);
                        let w = ((t1.wrapping_add((idx.wrapping_mul(2)) as u32))
                            as *const u16)
                            .read_unaligned();
                        let top = w >> 14;
                        if top == 1 {
                            let entry = ent.wrapping_add(
                                ((w & 0x3FFF) as u32).wrapping_mul(16),
                            );
                            if plane_hit(entry, ax, ay, az, bx, by, bz, lo_z, hi_z, out) {
                                return 1;
                            }
                        } else if top == 3 {
                            let mut ci = (w & 0x3FFF) as u32;
                            let first = ((t2.wrapping_add(ci.wrapping_mul(2)))
                                as *const u16)
                                .read_unaligned();
                            if first & 0xC000 != 0 {
                                let mut cw = first;
                                loop {
                                    if (cw >> 14) == 1 {
                                        let entry = ent.wrapping_add(
                                            ((cw & 0x3FFF) as u32).wrapping_mul(16),
                                        );
                                        if plane_hit(
                                            entry, ax, ay, az, bx, by, bz, lo_z, hi_z, out,
                                        ) {
                                            return 1;
                                        }
                                    }
                                    ci = ci.wrapping_add(1);
                                    cw = ((t2.wrapping_add(ci.wrapping_mul(2)))
                                        as *const u16)
                                        .read_unaligned();
                                    if cw & 0xC000 == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    gy = gy.wrapping_add(1);
                    if gy > hi_cy {
                        break;
                    }
                }
            }
            gx = gx.wrapping_add(1);
            base = base.wrapping_add(12);
            if gx > hi_cx {
                return 0;
            }
        }
    }
});
