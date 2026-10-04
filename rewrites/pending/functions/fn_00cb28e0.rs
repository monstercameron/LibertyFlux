// original: 0x00CB28E0 CTaskComplexMoveCrowdAroundLocation::vf19

/// Keep the ped milling around the crowd location while inside its radius.
///
/// `this` is the complex task, `ped` the ped. The crowd probe (callee 1)
/// runs first; when it says no, the fail-over subtask (0x3ae) is created.
/// Otherwise the crowd update (callee 2) and the position fixup (callee 3)
/// run, and when milling is enabled (`this+0x5a` non-zero) the distance
/// from the ped's position (`[ped+0x20]`, floats at +0x30/+0x34/+0x38) to
/// the crowd anchor (`this+0x20/+0x24/+0x28`) is compared against the
/// crowd radius (`this+0x50`): inside it (a NaN comparison also counts as
/// inside, matching the original's jump-if-below-or-equal) the mill-about
/// subtask (0x11a) is created, outside it the fail-over (0x3ae). Creation
/// goes through the factory (callee 4) as (id, ped). The sum-of-squares
/// order is the original's: x*x + y*y first, then + z*z, then the root.
///
/// Original: 0x00CB28E0 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB28E0(this: u32, ped: u32) -> u32 {
    unsafe {
        const CROWD_PROBE: u32 = 1;
        const CROWD_UPDATE: u32 = 2;
        const FIXUP_POS: u32 = 3;
        const CREATE_SUB: u32 = 4;
        const PED_MATRIX: u32 = 0x20;
        const ANCHOR_X: u32 = 0x20;
        const MILLING: u32 = 0x5a;
        const RADIUS: u32 = 0x50;
        const MILL_ABOUT: u32 = 0x11a;
        const FAIL_OVER: u32 = 0x3ae;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        if lf_checker_rt::callee_thiscall!(CROWD_PROBE, u32, this, ped) as u8 == 0 {
            return lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, FAIL_OVER, ped);
        }
        lf_checker_rt::callee_thiscall!(CROWD_UPDATE, u32, this, ped);
        lf_checker_rt::callee_thiscall!(FIXUP_POS, u32, this, ped);
        if (this.wrapping_add(MILLING) as *const u8).read_unaligned() == 0 {
            return lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, FAIL_OVER, ped);
        }
        let matrix = (ped.wrapping_add(PED_MATRIX) as *const u32).read_unaligned();
        let dx = sub(rdf(this.wrapping_add(ANCHOR_X)), rdf(matrix.wrapping_add(0x30)));
        let dy = sub(
            rdf(this.wrapping_add(ANCHOR_X + 4)),
            rdf(matrix.wrapping_add(0x34)),
        );
        let dz = sub(
            rdf(this.wrapping_add(ANCHOR_X + 8)),
            rdf(matrix.wrapping_add(0x38)),
        );
        let dist2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        let dist = dist2.sqrt();
        let radius = rdf(this.wrapping_add(RADIUS));
        // jbe after comiss: taken unless radius is strictly greater.
        let id = if !(radius > dist) { FAIL_OVER } else { MILL_ABOUT };
        lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, id, ped)
    }
});
