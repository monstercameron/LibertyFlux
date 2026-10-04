// original: 0x00cb5530 ped_task_row_copy_dispatch (proposed)

/// Copy a context row into the object, then dispatch a task request by code.
///
/// `this` is the requesting object, `arg1` a context whose row pointer at
/// `+0x20` yields four dwords (`+0x30`..`+0x3c`) copied to `this+0x90`..`+0x9c`
/// on every call, and `code` selects the path.
///
/// Paths: `0x204` returns a no-argument query on the task manager; `0x124`
/// returns a one-argument (0x32) query; `0x11a` picks 0x7d0 when the float at
/// `this+0x40` is zero (either sign) and 0x1f4 otherwise, NaN included, and
/// creates a default task from it; `0x389` creates a ten-argument task, then
/// re-copies the four dwords from a computed table entry
/// (`p + 16 * [p]`, `p = [this+0x64]`), clears three dwords and reinitializes
/// the seed fields; `0x3ae` scales two integer draws, negates a float draw,
/// transforms them against the context row and creates a ten-argument task
/// whose second argument points at the first computed float (two words are
/// already pushed when the address is taken, so it names the f0 slot), then
/// tags the result; `0x516` and any other code, or a null task manager,
/// return 0 (a null manager on the `0x3ae` path faults on a null write, like
/// the original).
///
/// The second and third computed floats never leave the original's frame,
/// so only the first is observed (through the snapped pointer argument).
/// The float operation order is the original's.
///
/// Original: 0x00cb5530 (thiscall, arg then code on the stack), returns the
/// query/creation call's result, or 0.
lf_checker_rt::export!(thiscall, rw_00cb5530(this: u32, arg1: u32, code: u32) -> u32 {
    unsafe {
        const ROW_OUT: u32 = 0x90;
        const F40: u32 = 0x40;
        const F18: u32 = 0x18;
        const TAB: u32 = 0x64;
        const FA4: u32 = 0xa4;
        const FA8: u32 = 0xa8;
        const GATE_CALLEE: u32 = 1;
        const Q0_CALLEE: u32 = 2;
        const Q1_CALLEE: u32 = 3;
        const MAKE_CALLEE: u32 = 4;
        const BIG_CALLEE: u32 = 5;
        const DRAW1_CALLEE: u32 = 6;
        const DRAW2_CALLEE: u32 = 7;
        const FDRAW_CALLEE: u32 = 8;
        const XF_CALLEE: u32 = 9;
        const FIN_CALLEE: u32 = 10;
        const TASK_MGR_GLOBAL: u32 = 0x167e2a0;
        const SEED_GLOBAL: u32 = 0x11735b4;
        const EIGHT: f32 = 8.0;
        const ONE: f32 = 1.0;
        const HALF: f32 = 0.5;
        const THREE: f32 = 3.0;
        const TWO_PI: f32 = 6.2831854820251465;
        const SCALE: f32 = f32::from_bits(0x38000100);
        const SIGN: u32 = 0x80000000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn gate() -> u32 {
            unsafe {
                let g = lf_checker_rt::global::<u32>(TASK_MGR_GLOBAL).read();
                lf_checker_rt::callee_thiscall!(GATE_CALLEE, u32, g)
            }
        }

        // Prologue row copy, runs on every path.
        let row = rd32(arg1 + 0x20);
        ((this + ROW_OUT) as *mut u32).write_unaligned(rd32(row + 0x30));
        ((this + ROW_OUT + 4) as *mut u32).write_unaligned(rd32(row + 0x34));
        ((this + ROW_OUT + 8) as *mut u32).write_unaligned(rd32(row + 0x38));
        ((this + ROW_OUT + 12) as *mut u32).write_unaligned(rd32(row + 0x3c));

        if code == 0x389 {
            let mgr = gate();
            let task = if mgr == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    BIG_CALLEE, u32, mgr,
                    rdf(this + F18).to_bits(), rd32(this + TAB), 0,
                    HALF.to_bits(), 0, 0, 1, 1, 0, (-1.0f32).to_bits()
                )
            };
            let p = rd32(this + TAB);
            let q = p.wrapping_add(rd32(p).wrapping_shl(4));
            ((this + ROW_OUT) as *mut u32).write_unaligned(rd32(q));
            ((this + ROW_OUT + 4) as *mut u32).write_unaligned(rd32(q + 4));
            ((this + ROW_OUT + 8) as *mut u32).write_unaligned(rd32(q + 8));
            ((this + ROW_OUT + 12) as *mut u32).write_unaligned(rd32(q + 12));
            ((this + 0x60) as *mut u32).write_unaligned(0);
            ((this + 0x3c) as *mut u32).write_unaligned(0);
            ((this + F40) as *mut u32).write_unaligned(0);
            let seed = lf_checker_rt::global::<u32>(SEED_GLOBAL).read();
            ((this + 0x30) as *mut u32).write_unaligned(seed);
            ((this + 0x34) as *mut u32).write_unaligned(0x3e8);
            ((this + 0x38) as *mut u8).write(1);
            return task;
        }
        if code == 0x11a {
            // 0x7d0 iff the float is zero (either sign); NaN takes 0x1f4.
            let pick = if rdf(this + F40) == 0.0 { 0x7d0 } else { 0x1f4 };
            let mgr = gate();
            if mgr == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(
                MAKE_CALLEE, u32, mgr, pick, 0, 0, EIGHT.to_bits()
            );
        }
        if code == 0x124 {
            let mgr = gate();
            if mgr == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(Q1_CALLEE, u32, mgr, 0x32);
        }
        if code == 0x204 {
            let mgr = gate();
            if mgr == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(Q0_CALLEE, u32, mgr);
        }
        if code == 0x3ae {
            ((this + 0x24) as *mut u32).write_unaligned(0);
            let a4 = rdf(this + FA4);
            let a8 = rdf(this + FA8);
            let d1: u32 = lf_checker_rt::callee_cdecl!(DRAW1_CALLEE, u32,);
            let d = sub(a8, a4);
            let mut e1 = (d1 as i32) as f32;
            e1 = mul(e1, SCALE);
            e1 = mul(e1, d);
            let s2c = add(e1, a4);
            let d2: u32 = lf_checker_rt::callee_cdecl!(DRAW2_CALLEE, u32,);
            let mut e2 = (d2 as i32) as f32;
            e2 = mul(e2, SCALE);
            let s28 = mul(e2, TWO_PI);
            let r1: u32 = lf_checker_rt::callee_cdecl!(FDRAW_CALLEE, u32,);
            let neg = f32::from_bits(r1 ^ SIGN);
            let r2: u32 = lf_checker_rt::callee_cdecl!(XF_CALLEE, u32, s28.to_bits());
            let r2f = f32::from_bits(r2);
            let t = mul(r2f, s2c);
            let u = mul(neg, s2c);
            let v = mul(s2c, 0.0);
            let f2 = add(rdf(row + 0x34), t);
            let f1 = add(rdf(row + 0x38), v);
            let f0 = add(rdf(row + 0x30), u);
            let mgr = gate();
            if mgr == 0 {
                (0xd8 as *mut u32).write_unaligned(0x4000000);
                return 0;
            }
            // The second argument points at the f0 slot: two words are
            // already pushed when the address is taken, so the slot the
            // original addresses holds f0, not f1.
            let r: u32 = lf_checker_rt::callee_thiscall!(
                FIN_CALLEE, u32, mgr,
                rdf(this + F18).to_bits(), core::ptr::addr_of!(f0) as u32,
                HALF.to_bits(), THREE.to_bits(),
                0xffffffff, 1, 0, 0, 0, 1
            );
            ((r + 0xd8) as *mut u32).write_unaligned(
                ((r + 0xd8) as *const u32).read_unaligned() | 0x4000000,
            );
            let seed = lf_checker_rt::global::<u32>(SEED_GLOBAL).read();
            ((this + 0x54) as *mut u32).write_unaligned(seed);
            ((this + 0x58) as *mut u32).write_unaligned(0xfa0);
            ((this + 0x5c) as *mut u8).write(1);
            return r;
        }
        0
    }
});
