// original: 0x00d108f0 task_aim_update (proposed)

/// Solve one aim-update step between two task objects, returning 1 when the
/// shot solution is accepted and 0 otherwise.
///
/// `owner` and `target` are task objects. Each holds a position-block pointer
/// at `+0x20` (three floats at `+0x30/+0x34/+0x38`) and a gate word at
/// `+0xd68`. `param` is an opaque word forwarded to the helpers. The target
/// additionally carries a flag byte at `+0x26c` (bit 2 selects an alternate
/// word at `+0xb30`).
///
/// Algorithm: when the target gate is non-zero, helper 0 refines the pair
/// from the difference vector and returns four words. The working source
/// point stays the source position (the original reloads it before
/// restoring the stack, so it re-reads the saved value); when helper 0
/// reports success the first three output words become the working aim
/// point, otherwise the target position is kept. From the aim-minus-source
/// difference the function forms the
/// normalized direction (a zero-length difference yields a zero direction,
/// not a division) and a range word `min(10, |source - aim| - 2)`, then runs
/// the acceptance chain: helper 1 (14 words: param, source position,
/// direction, two constants, the range word, more words), a per-owner
/// attach (helper 2), a gate check (helper 3), a per-gate attach
/// (helper 4), a second refinement from the current aim point (helper 7,
/// same helper as 0 at another site), and a final 6-word accept call
/// (helper 5). Any rejection runs the detach helper (6) and returns 0,
/// except a helper-1 rejection which returns 0 directly.
///
/// Float operation order is the original's (see the body). NaN inputs flow
/// through the same comparisons: an unordered length comparison takes the
/// reciprocal path, and an unordered range comparison yields 10.
///
/// Original: 0x00d108f0 (cdecl, three stack words, byte result in `al`).
lf_checker_rt::export!(cdecl, rw_00d108f0(param: u32, owner: u32, target: u32) -> u32 {
    unsafe {
        const POS_PTR: u32 = 0x20;
        const GATE: u32 = 0xd68;
        const PX: u32 = 0x30;
        const PY: u32 = 0x34;
        const PZ: u32 = 0x38;
        const FLAG_OFF: u32 = 0x26c;
        const FLAG_ALT_BIT: u8 = 4;
        const ALT_OFF: u32 = 0xb30;
        const HELPER_W0: u32 = 0x3fb0ef3c;
        const HELPER_W1: u32 = 0x40000000;
        const ACCEPT_TAG: u32 = 0x8e;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let esi = owner;
        let edi = target;
        // Source and target positions.
        let apos = rd32(esi + POS_PTR);
        let ax = rdf(apos + PX);
        let ay = rdf(apos + PY);
        let az = rdf(apos + PZ);
        let bpos = rd32(edi + POS_PTR);
        let bx = rdf(bpos + PX);
        let by = rdf(bpos + PY);
        let bz = rdf(bpos + PZ);
        // Working aim point (B') and working source point (A').
        let (bpx, bpy, bpz, apx, apy, apz);
        // Scratch reused like the original's frame slots: `slot60` is the
        // refinement output area, `slot40_fourth` the word past the aim point.
        let mut slot60 = [0u32; 4];
        let mut slot40_fourth = 0u32;
        let gate = rd32(edi + GATE);
        if gate != 0 {
            let dx = sub(ax, bx);
            let dy = sub(ay, by);
            let dz = sub(az, bz);
            let vin = [dx.to_bits(), dy.to_bits(), dz.to_bits()];
            let tmp = [0u32; 4];
            let al0: u32 = lf_checker_rt::callee_cdecl!(
                0, u32, gate, edi, vin.as_ptr() as u32, tmp.as_ptr() as u32,
                slot60.as_mut_ptr() as u32
            );
            // The original reloads the working source point BEFORE restoring
            // the stack (the add comes after the loads), so it re-reads the
            // saved source position, not the refinement output.
            apx = ax;
            apy = ay;
            apz = az;
            if (al0 & 0xff) != 0 {
                bpx = f32::from_bits(slot60[0]);
                bpy = f32::from_bits(slot60[1]);
                bpz = f32::from_bits(slot60[2]);
                slot40_fourth = slot60[3];
            } else {
                bpx = bx;
                bpy = by;
                bpz = bz;
            }
        } else {
            apx = ax;
            apy = ay;
            apz = az;
            bpx = bx;
            bpy = by;
            bpz = bz;
        }
        // Aim-minus-source difference and its squared length ((dy^2 + dx^2) + dz^2).
        let dx1 = sub(bpx, apx);
        let dy1 = sub(bpy, apy);
        let dz1 = sub(bpz, apz);
        let dist2 = add(add(mul(dy1, dy1), mul(dx1, dx1)), mul(dz1, dz1));
        // Zero length keeps a zero direction; otherwise scale by 1/sqrt.
        let inv = if dist2 == 0.0 {
            0.0
        } else {
            div(1.0, dist2.sqrt())
        };
        let dirx = mul(dx1, inv);
        let diry = mul(dy1, inv);
        let dirz = mul(dz1, inv);
        // Range word from fresh reversed subtractions, same sum order.
        let nx = sub(apx, bpx);
        let ny = sub(apy, bpy);
        let nz = sub(apz, bpz);
        let d2 = add(add(mul(ny, ny), mul(nx, nx)), mul(nz, nz));
        let ranged = sub(d2.sqrt(), 2.0);
        let w2 = if 10.0 > ranged { ranged } else { 10.0 };
        // Helper 1: param, source position, direction, constants, range, rest.
        let src_pos = [ax.to_bits(), ay.to_bits(), az.to_bits()];
        let dir = [dirx.to_bits(), diry.to_bits(), dirz.to_bits()];
        let snap_a4 = [
            dy1.to_bits(),
            bpx.to_bits(),
            bpy.to_bits(),
            bpz.to_bits(),
        ];
        let snap_a = (snap_a4.as_ptr() as u32).wrapping_add(4);
        let eax1: u32 = lf_checker_rt::callee_cdecl!(
            1, u32, param, src_pos.as_ptr() as u32, dir.as_ptr() as u32,
            HELPER_W0, HELPER_W1, w2.to_bits(), 0, 0,
            snap_a, 2, 0, 0, 0, 0
        );
        if eax1 == 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, esi, eax1);
        let esi_gate = rd32(esi + GATE);
        let al3: u32 = lf_checker_rt::callee_cdecl!(3, u32, esi, esi_gate, param);
        if (al3 & 0xff) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, esi);
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, esi_gate, esi);
        // Second refinement from the current aim point minus the source.
        let apos2 = rd32(esi + POS_PTR);
        let ax2 = rdf(apos2 + PX);
        let ay2 = rdf(apos2 + PY);
        let az2 = rdf(apos2 + PZ);
        let dx2 = sub(bpx, ax2);
        let dy2 = sub(bpy, ay2);
        let dz2 = sub(bpz, az2);
        let vin2 = [dx2.to_bits(), dy2.to_bits(), dz2.to_bits()];
        let al7: u32 = lf_checker_rt::callee_cdecl!(
            7, u32, esi_gate, esi, vin2.as_ptr() as u32, param,
            slot60.as_mut_ptr() as u32
        );
        if (al7 & 0xff) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, esi);
            return 0;
        }
        let flag = ((edi + FLAG_OFF) as *const u8).read();
        let alt = if flag & FLAG_ALT_BIT != 0 {
            rd32(edi + ALT_OFF)
        } else {
            0
        };
        let slot40 = [bpx.to_bits(), bpy.to_bits(), bpz.to_bits(), slot40_fourth];
        let al5: u32 = lf_checker_rt::callee_cdecl!(
            5, u32, slot60.as_ptr() as u32, slot40.as_ptr() as u32, alt,
            ACCEPT_TAG, 2, 7
        );
        if (al5 & 0xff) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, esi);
            return 0;
        }
        1
    }
});
