// original: 0x00cb2090 CTaskComplexFollowPatrolRoute::vf19

/// Pick the patrol-route waypoint the ped should head for, then ask the task
/// system to build the matching subtask.
///
/// `this` is the complex task: route array pointer at `+0x24`, flag word at
/// `+0x28`, chosen waypoint index (word) at `+0x1a`. `ped` is the ped; its
/// position vector lives at `ped[0x20] + 0x30`. The route holds a 16-bit
/// point count at offset 0 and one 16-byte point (x, y, z floats) per entry
/// starting at entry index 29, i.e. byte `(i + 29) * 16`.
///
/// Behaviour: clears flag bit 0. When the route's first dword is zero there
/// is no route, so it orders subtask `0xc8` and returns. Otherwise it scans
/// every segment from point `i` to point `(i + 1) % count`: the projection
/// of the ped position onto the segment direction (normalised by
/// `1 / length`, zero when the segment has zero length), keeping the
/// projection that falls strictly inside its segment (`0 < t < length`) and
/// is closest to the ped. When no projection qualifies it falls back to the
/// nearest route point. The winning index (or `0xffff` when the count is not
/// positive) is stored at `+0x1a`. Flag bit 1 then selects the subtask: set
/// orders `0x3ae`, clear asks callee 2 for the order id and passes it on.
/// Flag bits 0 and 1 are cleared. Returns the subtask id callee 1 answered.
///
/// Float order is the original's scalar-SSE order: segment length squared as
/// `(dy*dy + dx*dx) + dz*dz`, the projection parameter as
/// `(rx*nx + ry*ny) + rz*nz`, distances the same way, best tracked squared.
/// The zero-length test reproduces the original's `lahf` parity trick: the
/// reciprocal is taken exactly when the squared length is above zero or NaN.
///
/// Original: 0x00cb2090 (thiscall, one stack word; callee 1 is thiscall with
/// one stack word, callee 2 is thiscall with none).
lf_checker_rt::export!(thiscall, rw_00cb2090(this: u32, ped: u32) -> u32 {
    unsafe {
        const ROUTE_PTR: u32 = 0x24;
        const FLAGS: u32 = 0x28;
        const BEST_INDEX_OUT: u32 = 0x1a;
        const FIRST_ENTRY: i32 = 29;
        const ENTRY_BYTES: i32 = 16;
        const PED_POS_LINK: u32 = 0x20;
        const POS_LINK_BIAS: u32 = 0x30;
        const ORDER_EMPTY: u32 = 0xc8;
        const ORDER_FLAGGED: u32 = 0x3ae;
        const BEST_INIT: f32 = f32::from_bits(0x7f7f_ffff); // FLT_MAX
        const ONE: f32 = 1.0;
        const CALLEE_TASK: u32 = 1;
        const CALLEE_NEXT: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
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
        wr32(this + FLAGS, rd32(this + FLAGS) & 0xffff_fffe);
        let flags = rd32(this + FLAGS);
        if rd32(route) == 0 {
            return lf_checker_rt::callee_thiscall!(CALLEE_TASK, u32, this, ORDER_EMPTY);
        }
        let pos = rd32(ped + PED_POS_LINK).wrapping_add(POS_LINK_BIAS);
        let count = rd16(route) as i16 as i32;
        let mut best_next: i32 = -1;
        let mut best = BEST_INIT;
        if count > 0 {
            // Closest projection strictly inside a segment.
            let mut edi: i32 = 0;
            let mut idx: i32 = 0;
            loop {
                let rem_next = (idx + 1) % count;
                let next = (rem_next as u16) as i16 as i32;
                let rem_cur = idx % count;
                let cur = (rem_cur as u16) as i16 as i32;
                let b = route.wrapping_add((((next + FIRST_ENTRY) * 2) as u32).wrapping_mul(8));
                let a = route.wrapping_add((((cur + FIRST_ENTRY) * 2) as u32).wrapping_mul(8));
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
                        best_next = (rem_next as u16) as u32 as i32;
                        best = d2;
                    }
                }
                edi = edi.wrapping_add(1);
                idx = (edi as u16) as i16 as i32;
                if idx >= count {
                    break;
                }
            }
            best = BEST_INIT;
        }
        if best_next as u16 == 0xffff {
            // No projection qualified: nearest route point wins.
            if count > 0 {
                let (px0, py0, pz0) = (rdf(pos), rdf(pos + 4), rdf(pos + 8));
                let mut ecx: i32 = 0;
                let mut eax: i32 = 0;
                loop {
                    let e = route.wrapping_add(((eax + FIRST_ENTRY) * ENTRY_BYTES) as u32);
                    let dx = sub(px0, rdf(e));
                    let dy = sub(py0, rdf(e + 4));
                    let dz = sub(pz0, rdf(e + 8));
                    let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                    if mul(best, best) > d2 {
                        best_next = (ecx as u16) as u32 as i32;
                        best = d2;
                    }
                    ecx = ecx.wrapping_add(1);
                    eax = (ecx as u16) as i16 as i32;
                    if eax >= count {
                        break;
                    }
                }
            }
        }
        wr16(this + BEST_INDEX_OUT, best_next as u16);
        let cleared = flags & 0xffff_fffd;
        wr32(this + FLAGS, cleared);
        if flags & 2 != 0 {
            lf_checker_rt::callee_thiscall!(CALLEE_TASK, u32, this, ORDER_FLAGGED)
        } else {
            let nxt = lf_checker_rt::callee_thiscall!(CALLEE_NEXT, u32, this);
            lf_checker_rt::callee_thiscall!(CALLEE_TASK, u32, this, nxt)
        }
    }
});
