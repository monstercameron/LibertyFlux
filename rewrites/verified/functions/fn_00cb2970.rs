// original: 0x00cb2970 CTaskComplexMoveFollowPointRoute::vf19

/// Pick the point-route waypoint the ped should head for, then ask the task
/// system to build the matching subtask.
///
/// `this` is the complex task: route pointer at `+0x34`, chosen index at
/// `+0x38`, flag word at `+0x3c`, movement flag at `+0x20`. `ped` is the
/// ped; its position vector lives at `ped[0x20] + 0x30`. The route holds a
/// 32-bit point count at offset 0 and one 16-byte point (x, y, z floats) per
/// entry at byte `(i + 1) * 16`.
///
/// Behaviour: clears flag bit 1. When the stored index already equals the
/// route count there is nothing to search, so it orders subtask `0x11c` for
/// the ped and returns. When the ped's flag byte at `+0x26c` has bit 2 set
/// it returns 0. When the movement flag is clear it skips the search but
/// still builds the subtask below. Otherwise it scans every segment from
/// point `i` to point `(i + 1) % count` exactly like the patrol-route scan
/// (projection strictly inside the segment, closest to the ped wins; the
/// zero-length test takes the reciprocal exactly when the squared length is
/// above zero or NaN), falling back to the nearest route point when no
/// projection qualifies. The winning index, or -1 when the count is not
/// positive, is stored at `+0x38`. It then asks callee 2 for an order id and
/// orders it for the ped through callee 1, returning callee 1's answer.
///
/// Float order is the original's scalar-SSE order, shared with the
/// patrol-route scan: `(dy*dy + dx*dx) + dz*dz` for lengths and distances,
/// `(rx*nx + ry*ny) + rz*nz` for the projection parameter.
///
/// Original: 0x00cb2970 (thiscall, one stack word; callee 1 is thiscall with
/// two stack words, callee 2 is thiscall with none).
lf_checker_rt::export!(thiscall, rw_00cb2970(this: u32, ped: u32) -> u32 {
    unsafe {
        const ROUTE_PTR: u32 = 0x34;
        const INDEX_OUT: u32 = 0x38;
        const FLAGS: u32 = 0x3c;
        const MOVE_FLAG: u32 = 0x20;
        const PED_POS_LINK: u32 = 0x20;
        const POS_LINK_BIAS: u32 = 0x30;
        const PED_STATE: u32 = 0x26c;
        const ENTRY_BYTES: i32 = 16;
        const ORDER_DONE: u32 = 0x11c;
        const BEST_INIT: f32 = f32::from_bits(0x7f7f_ffff); // FLT_MAX
        const ONE: f32 = 1.0;
        const CALLEE_TASK: u32 = 1;
        const CALLEE_NEXT: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let route = rd32(this + ROUTE_PTR);
        wr32(this + FLAGS, rd32(this + FLAGS) & 0xffff_fffd);
        let count = rd32(route) as i32;
        if count == rd32(this + INDEX_OUT) as i32 {
            return lf_checker_rt::callee_thiscall!(CALLEE_TASK, u32, this, ORDER_DONE, ped);
        }
        if rd8(ped + PED_STATE) & 4 != 0 {
            return 0;
        }
        if rd32(this + MOVE_FLAG) == 0 {
            let nxt = lf_checker_rt::callee_thiscall!(CALLEE_NEXT, u32, this);
            return lf_checker_rt::callee_thiscall!(CALLEE_TASK, u32, this, nxt, ped);
        }
        let pos = rd32(ped + PED_POS_LINK).wrapping_add(POS_LINK_BIAS);
        let mut best_next: i32 = -1;
        let mut best = BEST_INIT;
        if count > 0 {
            // Closest projection strictly inside a segment.
            let mut idx: i32 = 0;
            loop {
                let next = (idx + 1) % count;
                let cur = idx % count;
                let b = route.wrapping_add(((next + 1) * ENTRY_BYTES) as u32);
                let a = route.wrapping_add(((cur + 1) * ENTRY_BYTES) as u32);
                let (bx, by, bz) = (rdf(b), rdf(b + 4), rdf(b + 8));
                let (ax, ay, az) = (rdf(a), rdf(a + 4), rdf(a + 8));
                let (dx, dy, dz) = (sub(bx, ax), sub(by, ay), sub(bz, az));
                let len2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                let seglen = core::hint::black_box(len2).sqrt();
                let inv = if len2 > 0.0 || len2.is_nan() { div(ONE, seglen) } else { 0.0 };
                let (nx, ny, nz) = (mul(dx, inv), mul(dy, inv), mul(dz, inv));
                let rx = sub(rdf(pos), ax);
                let rz = sub(rdf(pos + 8), az);
                let ry = sub(rdf(pos + 4), ay);
                let t = add(add(mul(rx, nx), mul(ry, ny)), mul(rz, nz));
                if t > 0.0 && seglen > t {
                    let px = add(ax, mul(nx, t));
                    let py = add(ay, mul(ny, t));
                    let pz = add(az, mul(nz, t));
                    let ex = sub(rdf(pos), px);
                    let ey = sub(rdf(pos + 4), py);
                    let ez = sub(rdf(pos + 8), pz);
                    let d2 = add(add(mul(ey, ey), mul(ex, ex)), mul(ez, ez));
                    if mul(best, best) > d2 {
                        best_next = next;
                        best = d2;
                    }
                }
                idx += 1;
                if idx >= count {
                    break;
                }
            }
            best = BEST_INIT;
        }
        if best_next != -1 {
            // A projection qualified; the point scan is skipped.
        } else if count > 0 {
            // No projection qualified: nearest route point wins.
            let (px0, py0, pz0) = (rdf(pos), rdf(pos + 4), rdf(pos + 8));
            let mut ecx: i32 = 0;
            loop {
                let e = route.wrapping_add(((ecx + 1) * ENTRY_BYTES) as u32);
                let dx = sub(px0, rdf(e));
                let dy = sub(py0, rdf(e + 4));
                let dz = sub(pz0, rdf(e + 8));
                let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                if mul(best, best) > d2 {
                    best_next = ecx;
                    best = d2;
                }
                ecx += 1;
                if ecx >= count {
                    break;
                }
            }
        }
        wr32(this + INDEX_OUT, best_next as u32);
        let nxt = lf_checker_rt::callee_thiscall!(CALLEE_NEXT, u32, this);
        lf_checker_rt::callee_thiscall!(CALLEE_TASK, u32, this, nxt, ped)
    }
});
